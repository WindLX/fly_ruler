using MarkupAttributes;
using UnityEngine;

namespace FlyRuler.Model.Scriptable
{
    [CreateAssetMenu(fileName = "ControlLimits", menuName = "FlyRuler/ControlLimits", order = 0)]
    public class ControlLimits : ScriptableObject
    {
        [Box("Thrust")]
        public double ThrustCmdLimitTop;
        public double ThrustCmdLimitBottom;
        public double ThrustRateLimit;

        [Box("Elevator")]
        public double EleCmdLimitTop;
        public double EleCmdLimitBottom;
        public double EleRateLimit;

        [Box("Ailerons")]
        public double AilCmdLimitTop;
        public double AilCmdLimitBottom;
        public double AilRateLimit;

        [Box("Rudder")]
        public double RudCmdLimitTop;
        public double RudCmdLimitBottom;
        public double RudRateLimit;
    }
}