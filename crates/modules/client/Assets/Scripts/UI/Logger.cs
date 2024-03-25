using System.Collections.Generic;
using FlyRuler.Base;
using PimDeWitte.UnityMainThreadDispatcher;
using TMPro;
using UnityEngine;

public class Logger : SingletonMono<Logger>
{
    private List<string> log = new();
    public TMP_Text text;

    public void Log(string msg)
    {
        UnityMainThreadDispatcher.Instance().Enqueue(() =>
        {
            if (Debug.isDebugBuild)
                Debug.Log(msg);
            log.Add(msg.ToString());
            if (log.Count > 10)
            {
                log.RemoveAt(0);
            }
            text.text = string.Join("\n", log);
        });
    }
}
