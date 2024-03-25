using CoreOutput;
using UnityEngine;

namespace FlyRuler.Entity
{
    public class Trace : MonoBehaviour
    {
        public GameObject body;
        public Transform bodyParent;
        private int count = 0;

        public void SetTransform(PlaneMessage value)
        {
            count++;
            var newPos = new Vector3((float)value.Output.State.Npos / 1000f,
                                     (float)value.Output.State.Altitude / 1000f,
                                     (float)value.Output.State.Epos / 1000f);
            if (count > 100)
            {
                count = 0;
                var newBody = Instantiate(body);
                newBody.transform.SetParent(bodyParent);
                newBody.transform.localPosition = transform.localPosition;
            }
            transform.localPosition = newPos;
        }

        public void Redraw()
        {
            for (int i = 0; i < bodyParent.childCount; i++)
            {
                var body = bodyParent.GetChild(i);
                Destroy(body.gameObject);
            }
        }
    }
}