using System.Collections.Generic;
using System;

namespace FlyRuler.Service
{
    public class EventBus
    {
        private static readonly EventBus instance = new();

#nullable enable
        private readonly Dictionary<string, List<Action<object>?>> MessageRecevier;
#nullable disable

        private EventBus()
        {
            MessageRecevier = new();
        }

        public static EventBus Instance
        {
            get { return instance; }
        }

        public void Publish(string eventName, ref Action<object> sendEventHandler)
        {
            if (MessageRecevier.ContainsKey(eventName) == false)
            {
                MessageRecevier.Add(eventName, new());
            }
            void bus(object message) => MessageRecevier[eventName]?.ForEach(re => re?.Invoke(message));
            sendEventHandler += bus;
        }

        public void Subscribe(string eventName, Action<object> receiveEventHandler)
        {
            if (MessageRecevier.ContainsKey(eventName) == true)
            {
                MessageRecevier[eventName].Add(receiveEventHandler);
            }
            else
            {
                MessageRecevier.Add(eventName, new() { receiveEventHandler });
            }
        }
    }
}
