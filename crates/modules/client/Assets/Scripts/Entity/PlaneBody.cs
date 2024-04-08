using System.Collections;
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

                StartCoroutine(Move(transform.position, newPos, Time.fixedDeltaTime));
                StartCoroutine(Rotate(transform.rotation, rotation, Time.fixedDeltaTime));
                lastUpdate = 0.0f;
            }
        }

        IEnumerator Move(Vector3 start, Vector3 end, float duration)
        {
            float time = 0;
            while (time < duration)
            {
                time += Time.deltaTime;
                var pos = Vector3.Lerp(start, end, time / duration);
                transform.position = pos;
                yield return null;
            }
        }

        IEnumerator Rotate(Quaternion start, Quaternion end, float duration)
        {
            float time = 0;
            while (time < duration)
            {
                time += Time.deltaTime;
                var angle = Quaternion.Lerp(start, end, time / duration);
                transform.rotation = angle;
                yield return null;
            }
        }
    }
}