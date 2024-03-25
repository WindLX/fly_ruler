using UnityEngine;

namespace FlyRuler.Base
{
    public abstract class LinearUIBase : MonoBehaviour
    {
        protected float value;
        public float Value
        {
            get { return value; }
            set { ValueSetter(value); }
        }

        protected abstract void ValueSetter(float value);
    }
}