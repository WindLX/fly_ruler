using DG.Tweening;
using FlyRuler.Base;
using UnityEngine;

namespace FlyRuler.UI
{
    public class AnglePanel : LinearUIBase
    {
        private float top;
        private float bottom;
        private float bais;
        private float weight;

        public RectTransform rectTransform;

        public void Init(float top, float bottom, float weight, float bais)
        {
            this.top = top;
            this.bottom = bottom;
            this.bais = bais;
            this.weight = weight;
        }

        protected override void ValueSetter(float value)
        {
            this.value = Mathf.Clamp(value, bottom, top);
            rectTransform.DOLocalRotate(new Vector3(0, 0, (this.value - bottom) / (top - bottom) * weight + bais), 0.01f);
        }
    }
}