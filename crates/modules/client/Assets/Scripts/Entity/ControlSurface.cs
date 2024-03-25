using System.Collections;
using FlyRuler.Base;
using UnityEngine;

namespace FlyRuler.Entity
{
    public class ControlSurface : LinearUIBase
    {
        public enum RotateAxis { X, Y, Z };
        private RotateAxis rotateAxis;
        private float top;
        private float bottom;
        private float weight;
        private IEnumerator currentCoroutine;

        public void Init(float top, float bottom, float weight, RotateAxis rotateAxis)
        {
            this.top = top;
            this.bottom = bottom;
            this.weight = weight;
            this.rotateAxis = rotateAxis;
        }

        protected override void ValueSetter(float value)
        {
            this.value = Mathf.Clamp(value, bottom, top);
            var angle = this.value * weight;
            var init = transform.localEulerAngles;
            if (init.x > 180)
            {
                init.x -= 360;
            }
            if (init.y > 180)
            {
                init.y -= 360;
            }
            if (init.z > 180)
            {
                init.z -= 360;
            }
            if (currentCoroutine != null)
            {
                StopCoroutine(currentCoroutine);
            }
            switch (rotateAxis)
            {
                case RotateAxis.X:
                    currentCoroutine = Rotate(init, new Vector3(angle, 0, 0), 0.01f);
                    break;
                case RotateAxis.Y:
                    currentCoroutine = Rotate(init, new Vector3(0, angle, 0), 0.01f);
                    break;
                case RotateAxis.Z:
                    currentCoroutine = Rotate(init, new Vector3(0, 0, angle), 0.01f);
                    break;
                default: break;
            }
            if (currentCoroutine != null)
            {
                StartCoroutine(currentCoroutine);
            }
        }

        IEnumerator Rotate(Vector3 start, Vector3 end, float duration)
        {
            float time = 0;
            while (time < duration)
            {
                time += Time.deltaTime;
                var angle = Vector3.Lerp(start, end, time / duration);
                transform.localEulerAngles = angle;
                yield return null;
            }
        }
    }
}