using CoreOutput;
using FlyRuler.Model.Scriptable;
using MarkupAttributes;
using UnityEngine;

namespace FlyRuler.Entity
{
    public class Plane : MonoBehaviour
    {
        private ControlLimits controlLimits = null;
        public ControlLimits ControlLimits
        {
            get => controlLimits;
            set
            {
                controlLimits = value;
                fire.Init((float)controlLimits.ThrustCmdLimitTop, (float)controlLimits.ThrustCmdLimitBottom);
                elevatorSurface.Init((float)controlLimits.EleCmdLimitTop, (float)controlLimits.EleCmdLimitBottom, -1.0f, ControlSurface.RotateAxis.Z);
                aileronRightSurface.Init((float)controlLimits.AilCmdLimitTop, (float)controlLimits.AilCmdLimitBottom, 1.0f, ControlSurface.RotateAxis.Z);
                aileronLeftSurface.Init((float)controlLimits.AilCmdLimitTop, (float)controlLimits.AilCmdLimitBottom, -1.0f, ControlSurface.RotateAxis.Z);
                rudderSurface.Init((float)controlLimits.RudCmdLimitTop, (float)controlLimits.RudCmdLimitBottom, -1.0f, ControlSurface.RotateAxis.Y);
            }
        }

        [Box("Entity")]
        public Fire fire;
        public PlaneBody planeBody;
        public ControlSurface elevatorSurface;
        public ControlSurface aileronRightSurface;
        public ControlSurface aileronLeftSurface;
        public ControlSurface rudderSurface;

        void Start()
        {
        }

        public void UpdateState(PlaneMessage planeMessage)
        {
            if (controlLimits == null)
                return;

            var control = planeMessage.Output.Control;

            fire.Value = (float)control.Thrust;
            elevatorSurface.Value = (float)control.Elevator;
            rudderSurface.Value = (float)control.Rudder;
            aileronRightSurface.Value = (float)control.Aileron;
            aileronLeftSurface.Value = (float)control.Aileron;

            planeBody.SetTransform(planeMessage.Output.State);
        }
    }
}