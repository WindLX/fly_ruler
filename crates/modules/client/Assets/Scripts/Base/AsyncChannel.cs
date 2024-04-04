using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Threading;
using Cysharp.Threading.Tasks;
// using UnityEngine;

namespace FlyRuler.Base
{
    public class AsyncChannel<T>
    {
        private readonly BlockingCollection<T> values = new(new ConcurrentQueue<T>());

        public AsyncChannel() { }

        public bool IsCompleted => values.IsCompleted;

        public void Write(T value, CancellationToken ct)
        {
            values.Add(value, ct);
        }

        public T Read(CancellationToken ct)
        {
            return values.Take(ct);
        }

        public bool TryRead(out T output)
        {
            return values.TryTake(out output);
        }

        public async UniTask WriteAsync(T value, CancellationToken cancellationToken)
        {
            var isSet = false;
            while (!isSet)
            {
                if (cancellationToken.IsCancellationRequested)
                {
                    throw new OperationCanceledException();
                }
                isSet = values.TryAdd(value);
                await UniTask.Yield();
            }
        }

        public async UniTask<T> ReadAsync(CancellationToken cancellationToken)
        {
            T value = default;
            var isGet = false;
            while (!isGet)
            {
                if (cancellationToken.IsCancellationRequested)
                {
                    throw new OperationCanceledException();
                }
                isGet = values.TryTake(out value);
                await UniTask.Yield();
            }
            return value;
        }

        public void Close()
        {
            values.CompleteAdding();
        }
    }
}