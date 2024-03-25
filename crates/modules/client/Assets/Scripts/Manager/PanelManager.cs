using CoreOutput;
using FlyRuler.Base;
using FlyRuler.Entity;
using FlyRuler.Model.Scriptable;
using FlyRuler.UI;
using MarkupAttributes;
using TMPro;
using UnityEngine;

namespace FlyRuler.Manager
{
    public class PanelManager : SingletonMono<PanelManager>
    {
        private ControlLimits controlLimits = null;
        public ControlLimits ControlLimits
        {
            get => controlLimits;
            set
            {
                controlLimits = value;

                thrustUI.Init((float)controlLimits.ThrustCmdLimitTop, (float)controlLimits.ThrustCmdLimitBottom);

                elevatorUI.Init((float)controlLimits.EleCmdLimitTop, (float)controlLimits.EleCmdLimitBottom, 90.0f, -90.0f);
                aileronUI.Init((float)controlLimits.AilCmdLimitTop, (float)controlLimits.AilCmdLimitBottom, -90.0f, 0.0f);
                rudderUI.Init((float)controlLimits.RudCmdLimitTop, (float)controlLimits.RudCmdLimitBottom, 90.0f, -90.0f);

                velocityUI.Init(2000.0f, 0, -90.0f, 0.0f);
            }
        }

        [Foldout("Top")]
        [TitleGroup("./Yaw")]
        public DirectionPanel directionPanel;
        public TMP_Text directionText;

        [Foldout("TopLeft")]
        [TitleGroup("./Pitch")]
        public PlanePointer phiPointer;
        public TMP_Text phiText;
        [TitleGroup("./Roll")]
        public PlanePointer thetaPointer;
        public TMP_Text thetaText;
        [TitleGroup("./AngleSpeed")]
        public TMP_Text pText;
        public TMP_Text qText;
        public TMP_Text rText;

        [Foldout("TopRight")]
        [TitleGroup("./Position")]
        public TMP_Text nposText;
        public TMP_Text eposText;
        public TMP_Text altText;

        [Foldout("Bottom")]
        [TitleGroup("./Speed")]
        public AnglePanel velocityUI;
        public TMP_Text velocityText;
        public TMP_Text machText;
        [TitleGroup("./Other")]
        public TMP_Text alphaText;
        public TMP_Text betaText;

        [Foldout("BottomRight")]
        [TitleGroup("./StateExtend")]
        public TMP_Text qbarText;
        public TMP_Text psText;
        public TMP_Text nxText;
        public TMP_Text nyText;
        public TMP_Text nzText;

        [Foldout("BottomLeft")]
        [TitleGroup("./Elevator")]
        public AnglePanel elevatorUI;
        public TMP_Text elevatorText;
        [TitleGroup("./Aileron")]
        public AnglePanel aileronUI;
        public TMP_Text aileronText;
        [TitleGroup("./Rudder")]
        public AnglePanel rudderUI;
        public TMP_Text rudderText;
        [TitleGroup("./Thrust")]
        public Bar thrustUI;
        public TMP_Text thrustText;

        public Trace trace;

        void Start()
        {
        }

        public void UpdateState(PlaneMessage planeMessage)
        {
            if (controlLimits == null)
                return;

            var control = planeMessage.Output.Control;
            thrustUI.Value = (float)control.Thrust;
            thrustText.text = control.Thrust.ToString("F2");

            elevatorUI.Value = (float)control.Elevator;
            elevatorText.text = control.Elevator.ToString("F4");

            rudderUI.Value = (float)control.Rudder;
            rudderText.text = control.Rudder.ToString("F4");

            aileronUI.Value = (float)control.Aileron;
            aileronText.text = control.Aileron.ToString("F4");

            velocityText.text = planeMessage.Output.State.Velocity.ToString("F4");
            velocityUI.Value = (float)planeMessage.Output.State.Velocity;

            directionPanel.Value = (float)planeMessage.Output.State.Psi;
            directionText.text = (planeMessage.Output.State.Psi * Mathf.Rad2Deg % 360).ToString("F4");
            phiText.text = (planeMessage.Output.State.Phi * Mathf.Rad2Deg % 360).ToString("F4");
            phiPointer.Value = (float)planeMessage.Output.State.Phi * Mathf.Rad2Deg;
            thetaText.text = (planeMessage.Output.State.Theta * Mathf.Rad2Deg % 360).ToString("F4");
            thetaPointer.Value = (float)planeMessage.Output.State.Theta * Mathf.Rad2Deg;

            machText.text = planeMessage.Output.StateExtend.Mach.ToString("F4");
            qbarText.text = planeMessage.Output.StateExtend.Qbar.ToString("F4");
            psText.text = planeMessage.Output.StateExtend.Ps.ToString("F2");
            nxText.text = planeMessage.Output.StateExtend.Nx.ToString("F4");
            nyText.text = planeMessage.Output.StateExtend.Ny.ToString("F4");
            nzText.text = planeMessage.Output.StateExtend.Nz.ToString("F4");

            alphaText.text = (planeMessage.Output.State.Alpha * Mathf.Rad2Deg).ToString("F4");
            betaText.text = (planeMessage.Output.State.Beta * Mathf.Rad2Deg).ToString("F4");
            pText.text = planeMessage.Output.State.P.ToString("F4");
            qText.text = planeMessage.Output.State.Q.ToString("F4");
            rText.text = planeMessage.Output.State.R.ToString("F4");

            nposText.text = planeMessage.Output.State.Npos.ToString("F4");
            eposText.text = planeMessage.Output.State.Epos.ToString("F4");
            altText.text = planeMessage.Output.State.Altitude.ToString("F4");

            trace.SetTransform(planeMessage);
        }
    }
}