using FlyRuler.Model.Scriptable;
using UnityEngine;
using FlyRuler.Base;
using System.Collections.Generic;
using CoreOutput;
using FlyRuler.Service;
using Cinemachine;

namespace FlyRuler.Manager
{
    public class PlaneManager : SingletonMono<PlaneManager>
    {
        public ControlLimits controlLimits;
        public GameObject planePrefab;
        public CinemachineVirtualCamera cinemachineVirtualCamera;

        private string selfId = null;
        private Entity.Plane selfObj = null;
        private Dictionary<string, Entity.Plane> planeObjs = new();

        void Start()
        {
            RPCClient.Instance.onLostPlane += LostPlaneHandler;
            RPCClient.Instance.onNewPlane += NewPlaneHandler;
            RPCClient.Instance.onPlaneMessageUpdate += PlaneMessageUpdateHandler;
            RPCClient.Instance.onDisconnected += DisconnectHandler;
            ControllerManager.Instance.onRawControlUpdate += RawControlUpdateHandler;
            ControllerManager.Instance.onSetSelf += SetSelfHandler;
        }

        public void SetSelfHandler(Id.Id selfId)
        {
            this.selfId = selfId.Id_;
            if (selfObj == null)
            {
                var obj = Instantiate(planePrefab);
                selfObj = obj.GetComponent<Entity.Plane>();
                selfObj.ControlLimits = controlLimits;
                PanelManager.Instance.ControlLimits = controlLimits;
                cinemachineVirtualCamera.Follow = obj.transform;
                cinemachineVirtualCamera.LookAt = obj.transform;
            }
        }

        public void DisconnectHandler()
        {
            selfId = null;
            Destroy(selfObj.gameObject);
            selfObj = null;
        }

        public void NewPlaneHandler(Id.Id id)
        {
            if (selfId == null || id.Id_ == selfId)
                return;

            var obj = Instantiate(planePrefab).GetComponent<Entity.Plane>();
            obj.ControlLimits = controlLimits;
            planeObjs.Add(id.Id_, obj);
        }

        public void LostPlaneHandler(Id.Id id)
        {
            if (planeObjs.ContainsKey(id.Id_))
            {
                Destroy(planeObjs[id.Id_].gameObject);
                planeObjs.Remove(id.Id_);
            }
        }

        public void PlaneMessageUpdateHandler(PlaneMessage planeMessage)
        {
            if (selfId == null || selfObj == null)
                return;

            if (planeMessage.Id.Id_ == selfId)
            {
                selfObj.UpdateState(planeMessage);
                PanelManager.Instance.UpdateState(planeMessage);
            }
            else if (planeObjs.ContainsKey(planeMessage.Id.Id_))
            {
                planeObjs[planeMessage.Id.Id_].UpdateState(planeMessage);
            }
            else
            {
                NewPlaneHandler(planeMessage.Id);
            }
        }

        public void RawControlUpdateHandler(Control.Control control)
        {
            control.Thrust = control.Thrust * (controlLimits.ThrustCmdLimitTop - controlLimits.ThrustCmdLimitBottom) + controlLimits.ThrustCmdLimitBottom;
            control.Elevator = control.Elevator * (controlLimits.EleCmdLimitTop - controlLimits.EleCmdLimitBottom) / 2 + (controlLimits.EleCmdLimitTop + controlLimits.EleCmdLimitBottom) / 2;
            control.Aileron = control.Aileron * (controlLimits.AilCmdLimitTop - controlLimits.AilCmdLimitBottom) / 2 + (controlLimits.AilCmdLimitTop + controlLimits.AilCmdLimitBottom) / 2;
            control.Rudder = control.Rudder * (controlLimits.RudCmdLimitTop - controlLimits.RudCmdLimitBottom) / 2 + (controlLimits.RudCmdLimitTop + controlLimits.RudCmdLimitBottom) / 2;
            RPCClient.Instance.SendControl(control);
        }
    }
}