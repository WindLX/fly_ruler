using System.IO;
using System.Linq;
using System.Net.Sockets;
using CoreOutput;
using Google.Protobuf;
using UnityEngine;
using System;
using Tomlyn;
using Tomlyn.Model;
using Cysharp.Threading.Tasks;
using Service;
using System.Collections.Generic;
using Plugin;
using System.Threading;
using FlyRuler.Base;

namespace FlyRuler.Service
{
    public class RPCClient : SingletonMono<RPCClient>
    {
        public Action<PlaneMessage> onPlaneMessageUpdate;
        public Action<Id.Id> onLostPlane;
        public Action<Id.Id> onNewPlane;
        public Action onDisconnected;
        public bool IsConnected { get; private set; }
        public string host = "127.0.0.1";
        public int port = 2350;

        private const int MAX_BUFFER = 1024 * 1024;
        private byte[] buffer = new byte[MAX_BUFFER];

        private TcpClient client = new();
        private CancellationTokenSource cts = new();
        private Channel<byte[]> channel = Channel.CreateSingleConsumerUnbounded<byte[]>();
        private Channel<GetModelInfosResponse> getModelInfosChannel = Channel.CreateSingleConsumerUnbounded<GetModelInfosResponse>();
        private Channel<PushPlaneResponse> pushPlaneChannel = Channel.CreateSingleConsumerUnbounded<PushPlaneResponse>();
        private UniTask recvHandler = new();
        private UniTask sendHandler = new();
        private UniTask tickHandler = new();

        protected override void Awake()
        {
            base.Awake();
            IsConnected = false;
        }

        void Start()
        {
            string configFileUrl = Application.streamingAssetsPath + "/config.toml";
            if (File.Exists(configFileUrl))
            {
                using StreamReader sr = new(configFileUrl);
                var configStr = sr.ReadToEnd();
                var config = Toml.ToModel(configStr);
                try
                {
                    host = (string)((TomlTable)config["server"])["host"];
                    port = (int)(long)((TomlTable)config["server"])["port"];
                }
                catch (Exception ex)
                {
                    Logger.Instance.Log(ex.ToString());
                }
            }
        }

        public async UniTask<bool> Connect()
        {
            Logger.Instance.Log($"Try to connect {host}:{port}");
            client ??= new();
            if (client.Connected)
            {
                IsConnected = true;
                return true;
            }

            var count = 0;
            while (true)
            {
                if (count == 0)
                {
                    cts = new();
                }
                if (cts.Token.IsCancellationRequested)
                {
                    throw new OperationCanceledException();
                }
                try
                {
                    await client.ConnectAsync(host, port);
                    break;
                }
                catch (Exception)
                {
                    client.Close();
                    client = new TcpClient();
                    count++;

                    Logger.Instance.Log($"Failed to connect {host}:{port}, retry {count}");
                    if (count >= 10)
                    {
                        Logger.Instance.Log($"Failed to connect {host}:{port}");
                        IsConnected = false;
                        return false;
                    }
                    await UniTask.Delay(TimeSpan.FromSeconds(6), ignoreTimeScale: false, cancellationToken: cts.Token);

                }
                await UniTask.Yield();
            }

            Logger.Instance.Log($"Connected to {host}:{port}");

            recvHandler = UniTask.RunOnThreadPool(async () =>
            {
                await ReceiveMsgHandler();
            });
            sendHandler = UniTask.RunOnThreadPool(async () =>
            {
                await WriteMsgHandler(channel.Reader);
            });
            tickHandler = UniTask.RunOnThreadPool(async () =>
            {
                while (true)
                {
                    if (cts.Token.IsCancellationRequested)
                    {
                        break;
                    }
                    Tick();
                    await UniTask.Delay(TimeSpan.FromMilliseconds(1000), ignoreTimeScale: false, cancellationToken: cts.Token);
                }
            });
            IsConnected = true;
            return true;
        }

        public void Disconnect()
        {
            cts.Cancel();
            IsConnected = false;
            onDisconnected?.Invoke();
            if (client == null)
            {
                client = new();
                return;
            }
            if (client.Connected)
            {
                client.Close();
                client.Dispose();
                client = new();
                Logger.Instance.Log("Disconnected");
            }
        }

        private async UniTask ReceiveMsgHandler()
        {
            var stream = client.GetStream();
            while (true)
            {
                if (cts.Token.IsCancellationRequested)
                {
                    throw new OperationCanceledException();
                }
                try
                {
                    var count = await stream.ReadAsync(buffer, 0, buffer.Length, cancellationToken: cts.Token);
                    ServiceCallResponse serviceCallResponse = ServiceCallResponse.Parser.ParseFrom(buffer.Take(count).ToArray());
                    if (serviceCallResponse.ResponseCase == ServiceCallResponse.ResponseOneofCase.GetModelInfos)
                    {
                        while (true)
                        {
                            if (cts.Token.IsCancellationRequested)
                            {
                                throw new OperationCanceledException();
                            }
                            try
                            {
                                getModelInfosChannel.Writer.TryWrite(serviceCallResponse.GetModelInfos);
                                break;
                            }
                            catch (Exception)
                            {
                                await UniTask.Yield();
                            }
                        }
                    }
                    else if (serviceCallResponse.ResponseCase == ServiceCallResponse.ResponseOneofCase.PushPlane)
                    {
                        while (true)
                        {
                            if (cts.Token.IsCancellationRequested)
                            {
                                throw new OperationCanceledException();
                            }
                            try
                            {
                                pushPlaneChannel.Writer.TryWrite(serviceCallResponse.PushPlane);
                                break;
                            }
                            catch (Exception)
                            {
                                await UniTask.Yield();
                            }
                        }
                    }
                    else if (serviceCallResponse.ResponseCase == ServiceCallResponse.ResponseOneofCase.Output)
                    {
                        onPlaneMessageUpdate?.Invoke(serviceCallResponse.Output);
                    }
                    else if (serviceCallResponse.ResponseCase == ServiceCallResponse.ResponseOneofCase.Error)
                    {
                        Logger.Instance.Log(serviceCallResponse.Error);
                    }
                    else if (serviceCallResponse.ResponseCase == ServiceCallResponse.ResponseOneofCase.LostPlane)
                    {
                        onLostPlane?.Invoke(serviceCallResponse.LostPlane);
                    }
                    else if (serviceCallResponse.ResponseCase == ServiceCallResponse.ResponseOneofCase.NewPlane)
                    {
                        onNewPlane?.Invoke(serviceCallResponse.NewPlane);
                    }
                }
                catch (Exception ex)
                {
                    Debug.Log(ex);
                    client.Close();
                    break;
                }
                await UniTask.Yield();
            }
        }

        private async UniTask WriteMsgHandler(ChannelReader<byte[]> channelReader)
        {
            var stream = client.GetStream();
            while (true)
            {
                if (cts.Token.IsCancellationRequested)
                {
                    throw new OperationCanceledException();
                }
                try
                {
                    var data = await channelReader.ReadAsync(cts.Token);
                    await stream.WriteAsync(data, 0, data.Length);
                }
                catch (Exception ex)
                {
                    Debug.Log(ex);
                    client.Close();
                    break;
                }
                await UniTask.Yield();
            }
        }

        public async UniTask<List<PluginInfoTuple>> GetModelInfos()
        {
            var serviceCall = new ServiceCall
            {
                Name = "GetModelInfos",
                GetModelInfos = new Google.Protobuf.WellKnownTypes.Empty()
            };
            using var ms = new MemoryStream();
            serviceCall.WriteTo(ms);
            var data = ms.ToArray();
            while (true)
            {
                if (cts.Token.IsCancellationRequested)
                {
                    throw new OperationCanceledException();
                }
                try
                {
                    channel.Writer.TryWrite(data);
                    break;
                }
                catch (Exception)
                {
                    await UniTask.Yield();
                }
            }
            var res = await getModelInfosChannel.Reader.ReadAsync(cancellationToken: cts.Token);
            var models = res.ModelInfos;
            return models.ToList();
        }

        public async UniTask<Id.Id> PushPlane(Id.Id modelId, PlaneInitCfg.PlaneInitCfg planeInitCfg)
        {
            var serviceCall = new ServiceCall
            {
                Name = "PushPlane",
                PushPlane = new PushPlaneRequest()
                {
                    ModelId = modelId,
                    PlaneInitCfg = planeInitCfg
                }
            };
            using var ms = new MemoryStream();
            serviceCall.WriteTo(ms);
            var data = ms.ToArray();
            while (true)
            {
                if (cts.Token.IsCancellationRequested)
                {
                    throw new OperationCanceledException();
                }
                try
                {
                    channel.Writer.TryWrite(data);
                    break;
                }
                catch (Exception)
                {
                    await UniTask.Yield();
                }
            }
            var res = await pushPlaneChannel.Reader.ReadAsync(cts.Token);
            return res.PlaneId;
        }

        public void SendControl(Control.Control control)
        {
            var serviceCall = new ServiceCall
            {
                Name = "SendControl",
                SendControl = new SendControlRequest()
                {
                    Control = control
                }
            };
            using var ms = new MemoryStream();
            serviceCall.WriteTo(ms);
            var data = ms.ToArray();
            channel.Writer.TryWrite(data);
        }

        private void Tick()
        {
            var serviceCall = new ServiceCall
            {
                Name = "Tick",
                Tick = new Google.Protobuf.WellKnownTypes.Empty()
            };
            using var ms = new MemoryStream();
            serviceCall.WriteTo(ms);
            var data = ms.ToArray();
            while (!channel.Writer.TryWrite(data)) { }
        }
    }
}
