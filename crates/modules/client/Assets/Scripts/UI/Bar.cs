using DG.Tweening;
using FlyRuler.Base;
using UnityEngine;

namespace FlyRuler.UI
{
    public class Bar : LinearUIBase
    {
        private float top;
        private float bottom;

        public RectTransform rectTransform;

        public void Init(float top, float bottom)
        {
            this.top = top;
            this.bottom = bottom;
        }

        protected override void ValueSetter(float value)
        {
            this.value = Mathf.Clamp(value, bottom, top);
            rectTransform.DOScaleY((this.value - bottom) / (top - bottom), 0.01f);
        }
    }
}