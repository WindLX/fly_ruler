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
    public class RPCClientAsync : SingletonMono<RPCClientAsync>
    {
        public Action<PlaneMessage> onPlaneMessageUpdate;
        public Action<Id.Id> onLostPlane;
        public Action<Id.Id> onNewPlane;
        public Action onDisconnected;
        public string host = "127.0.0.1";
        public int port = 2350;

        private const int MaxBufferSize = 1024 * 1024;
        private byte[] buffer = new byte[MaxBufferSize];
        private int bufferLength = 0;

        private TcpClient client = new();
        private CancellationTokenSource cts = new();
        private AsyncChannel<byte[]> channel = new();
        private AsyncChannel<GetModelInfosResponse> getModelInfosChannel = new();
        private AsyncChannel<PushPlaneResponse> pushPlaneChannel = new();
        private UniTask recvHandler = new();
        private UniTask sendHandler = new();
        private UniTask tickHandler = new();

        protected override void Awake()
        {
            base.Awake();
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
            if (IsSocketConnect(client))
            {
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
                        return false;
                    }
                    await UniTask.Delay(TimeSpan.FromSeconds(6), ignoreTimeScale: false, cancellationToken: cts.Token);

                }
                await UniTask.Yield();
            }

            Logger.Instance.Log($"Connected to {host}:{port}");

            channel = new();
            getModelInfosChannel = new();
            pushPlaneChannel = new();

            recvHandler = UniTask.RunOnThreadPool(async () =>
            {
                await ReceiveMsgHandler();
            });
            sendHandler = UniTask.RunOnThreadPool(async () =>
            {
                await WriteMsgHandler();
            });
            tickHandler = UniTask.RunOnThreadPool(async () =>
            {
                while (true)
                {
                    if (cts.Token.IsCancellationRequested)
                    {
                        break;
                    }
                    await Tick();
                    await UniTask.Delay(TimeSpan.FromMilliseconds(1000), ignoreTimeScale: false, cancellationToken: cts.Token);
                }
            });
            return true;
        }

        public void Disconnect()
        {
            cts.Cancel();
            if (IsSocketConnect(client))
            {
                var serviceCall = new ServiceCall
                {
                    Name = "Disconnect",
                    Disconnect = new Google.Protobuf.WellKnownTypes.Empty()
                };
                var tick = SeriveCallToBytes(serviceCall);
                client.Client.Blocking = false;
                client.Client.Send(tick, tick.Length, 0);

                channel.Close();
                channel = null;
                getModelInfosChannel.Close();
                getModelInfosChannel = null;
                pushPlaneChannel.Close();
                pushPlaneChannel = null;

                client.GetStream().Close();

                client.Close();
                client.Dispose();
                client = null;

                Logger.Instance.Log("Disconnected");
            }
            onDisconnected?.Invoke();
        }

        private static bool IsSocketConnect(TcpClient client)
        {
            if (client == null || client.Client == null)
            {
                return false;
            }
            if (client.Client.Connected == false || client.Client.RemoteEndPoint == null)
            {
                return false;
            }
            bool blockingState = client.Client.Blocking;
            try
            {
                var serviceCall = new ServiceCall
                {
                    Name = "Tick",
                    Tick = new Google.Protobuf.WellKnownTypes.Empty()
                };
                var tick = SeriveCallToBytes(serviceCall);
                client.Client.Blocking = false;
                client.Client.Send(tick, tick.Length, 0);
                return true;
            }
            catch (SocketException e)
            {
                if (e.NativeErrorCode.Equals(10035))
                    return true;
                else
                    return false;
            }
            finally
            {
                client.Client.Blocking = blockingState;
            }
        }

        private async UniTask<ServiceCallResponse> ReadServiceCallResponseAsync(Stream stream, CancellationToken cancellationToken)
        {
            while (true)
            {
                if (cancellationToken.IsCancellationRequested)
                {
                    throw new OperationCanceledException();
                }

                int bytesRead = await stream.ReadAsync(buffer, bufferLength, MaxBufferSize - bufferLength, cancellationToken);
                bufferLength += bytesRead;

                ServiceCallResponse serviceCallResponse = TryParseServiceCallResponse();
                if (serviceCallResponse != null)
                {
                    return serviceCallResponse;
                }
                else
                {
                    await UniTask.Yield();
                }
            }
        }

        private ServiceCallResponse TryParseServiceCallResponse()
        {
            if (bufferLength < 4)
            {
                return null;
            }

            var head = buffer[0..4];
            if (BitConverter.IsLittleEndian)
                Array.Reverse(head);
            int bodyLength = (int)BitConverter.ToUInt32(head, 0);

            if (bufferLength < 4 + bodyLength)
            {
                return null;
            }

            ServiceCallResponse serviceCallResponse = ServiceCallResponse.Parser.ParseFrom(buffer, 4, bodyLength);

            int remainingBytes = bufferLength - (4 + bodyLength);
            Array.Copy(buffer, 4 + bodyLength, buffer, 0, remainingBytes);
            bufferLength = remainingBytes;

            return serviceCallResponse;
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
                    var serviceCallResponse = await ReadServiceCallResponseAsync(stream, cts.Token);
                    if (serviceCallResponse != null)
                    {
                        if (serviceCallResponse.ResponseCase == ServiceCallResponse.ResponseOneofCase.GetModelInfos)
                        {
                            await getModelInfosChannel.WriteAsync(serviceCallResponse.GetModelInfos, cts.Token);
                        }
                        else if (serviceCallResponse.ResponseCase == ServiceCallResponse.ResponseOneofCase.PushPlane)
                        {
                            await pushPlaneChannel.WriteAsync(serviceCallResponse.PushPlane, cts.Token);
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

        private async UniTask WriteMsgHandler()
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
                    var data = await channel.ReadAsync(cts.Token);
                    stream.Write(data, 0, data.Length);
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

            await channel.WriteAsync(SeriveCallToBytes(serviceCall), cts.Token);

            var res = await getModelInfosChannel.ReadAsync(cancellationToken: cts.Token);
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

            await channel.WriteAsync(SeriveCallToBytes(serviceCall), cts.Token);
            var res = await pushPlaneChannel.ReadAsync(cts.Token);
            return res.PlaneId;
        }

        public async void SendControl(Id.Id planeId, Control.Control control)
        {
            var serviceCall = new ServiceCall
            {
                Name = "SendControl",
                SendControl = new SendControlRequest()
                {
                    PlaneId = planeId,
                    Control = control
                }
            };

            await channel.WriteAsync(SeriveCallToBytes(serviceCall), cts.Token);
        }

        private async UniTask Tick()
        {
            var serviceCall = new ServiceCall
            {
                Name = "Tick",
                Tick = new Google.Protobuf.WellKnownTypes.Empty()
            };

            await channel.WriteAsync(SeriveCallToBytes(serviceCall), cts.Token);
        }

        private static byte[] SeriveCallToBytes(ServiceCall serviceCall)
        {
            using var ms = new MemoryStream();
            serviceCall.WriteTo(ms);
            var data = ms.ToArray();

            byte[] message = new byte[data.Length + 4];
            var head = BitConverter.GetBytes(data.Length);
            if (BitConverter.IsLittleEndian)
                Array.Reverse(head);
            head.CopyTo(message, 0);
            data.CopyTo(message, 4);
            return message;
        }
    }
}
