using UnityEngine;

namespace FlyRuler.Entity
{
    public class PlaneBody : MonoBehaviour
    {
        private float lastUpdate = 0.0f;
        public void SetTransform(State.State value)
        {
            lastUpdate += Time.deltaTime;
            if (lastUpdate > 0.01f)
            {
                var rotation = Quaternion.Euler(new Vector3(
                    -(float)value.Theta * Mathf.Rad2Deg,
                    (float)value.Psi * Mathf.Rad2Deg,
                    (float)value.Phi * Mathf.Rad2Deg));

                var newPos = new Vector3(
                    (float)value.Epos / 3.048f,
                    (float)value.Altitude / 3.048f,
                    (float)value.Npos / 3.048f);

                transform.position = newPos;
                transform.rotation = rotation;
                lastUpdate = 0.0f;
            }
        }
    }
}