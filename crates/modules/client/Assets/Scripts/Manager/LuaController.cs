using System;
using CoreOutput;
using FlyRuler.Manager;
using FlyRuler.Service;
using UnityEngine;
using XLua;

public class LuaController : MonoBehaviour
{
    private int scriptControl = 0;
    private LuaEnv luaenv = new();

    void Start()
    {
        ControllerManager.Instance.onScriptControlUpdate += (id) => scriptControl = id;
        PlaneManager.Instance.onSelfPlaneMessageUpdate += scriptControlHandler;
        luaenv.Dispose();
    }

    private void scriptControlHandler(PlaneMessage message)
    {
        var id = PlaneManager.Instance.SelfId;
        if (id != null)
        {
            // RPCClientAsync.Instance.SendControl(new Id.Id() { Id_ = id }, control);
        }

    }

    void FixedUpdate()
    {
        if (scriptControl != 0)
        {

        }
    }
}
