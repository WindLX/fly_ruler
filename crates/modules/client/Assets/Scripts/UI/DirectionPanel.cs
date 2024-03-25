using FlyRuler.Base;
using UnityEngine;

namespace FlyRuler.UI
{
    public class DirectionPanel : LinearUIBase
    {
        public Material material;

        protected override void ValueSetter(float value)
        {
            float endValue = (float)(value * Mathf.Rad2Deg / 360 - 0.2215);
            material.SetVector("_Offset", new Vector2(endValue, 0));
        }
    }
}