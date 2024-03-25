using FlyRuler.Base;
using UnityEngine;

namespace FlyRuler.UI
{
    public class PlanePointer : LinearUIBase
    {
        public RectTransform rectTransform;

        protected override void ValueSetter(float value)
        {
            rectTransform.localEulerAngles = new Vector3(0, 0, value);
        }
    }
}