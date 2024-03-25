using UnityEngine;

[ExecuteInEditMode]
public class CameraFollower : MonoBehaviour
{
    public Transform target;
    public Vector3 offset;
    public float damping;

    void LateUpdate()
    {
        Vector3 targetPosition = target.position + target.TransformDirection(offset);
        transform.position = Vector3.Lerp(transform.position, targetPosition, Time.deltaTime * damping);
        transform.LookAt(target.position);
    }
}
