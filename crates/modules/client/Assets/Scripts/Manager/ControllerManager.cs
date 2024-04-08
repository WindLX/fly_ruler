using System;
using UnityEngine;
using UnityEngine.InputSystem;
using FlyRuler.Service;
using Cysharp.Threading.Tasks;
using FlyRuler.Base;
using MarkupAttributes;

namespace FlyRuler.Manager
{
    public class ControllerManager : SingletonMono<ControllerManager>
    {
        public Action<Control.Control> onRawControlUpdate;
        public Action<Id.Id> onSetSelf;


        [Box("Controller Curve")]

        public AnimationCurve elevatorSensitivityCurve;

        public AnimationCurve aileronSensitivityCurve;

        public AnimationCurve rudderSensitivityCurve;

        private Control.Control control = new();

        private bool firstConfirm = true;
        private bool firstCancel = true;

        void FixedUpdate()
        {
            onRawControlUpdate?.Invoke(control.Clone());
        }

        public void Thrust(InputAction.CallbackContext context)
        {
            control.Thrust = context.ReadValue<float>();
        }

        public void Elevator(InputAction.CallbackContext context)
        {
            var input = context.ReadValue<float>();
            var fixInput = elevatorSensitivityCurve.Evaluate(Mathf.Abs(input));
            if (input < 0)
            {
                fixInput = -fixInput;
            }
            var elevator = fixInput;
            control.Elevator = elevator;
        }

        public void Rudder(InputAction.CallbackContext context)
        {
            var input = context.ReadValue<float>();
            var fixInput = rudderSensitivityCurve.Evaluate(Mathf.Abs(input));
            if (input < 0)
            {
                fixInput = -fixInput;
            }
            var rudder = fixInput;
            control.Rudder = -rudder;
        }

        public void Ailerons(InputAction.CallbackContext context)
        {
            var input = context.ReadValue<float>();
            var fixInput = aileronSensitivityCurve.Evaluate(Mathf.Abs(input));
            if (input < 0)
            {
                fixInput = -fixInput;
            }
            var ailerons = fixInput;
            control.Aileron = ailerons;
        }

        public void Confirm(InputAction.CallbackContext context)
        {
            if (firstConfirm && PlaneManager.Instance.SelfId == null)
            {
                UniTask.RunOnThreadPool(async () =>
                {
                    // TODO a model manger and panel
                    await RPCClientAsync.Instance.Connect();
                    var modelInfos = await RPCClientAsync.Instance.GetModelInfos();
                    var id = await RPCClientAsync.Instance.PushPlane(modelInfos[0].Id, null);
                    onSetSelf?.Invoke(id);
                });
                firstConfirm = false;
                UniTask.RunOnThreadPool(async () =>
                {
                    await UniTask.Delay(TimeSpan.FromMilliseconds(1000));
                    firstConfirm = true;
                });
            }
        }

        public void Cancel(InputAction.CallbackContext context)
        {
            if (firstCancel)
            {
                RPCClientAsync.Instance.Disconnect();
                firstCancel = false;
                UniTask.RunOnThreadPool(async () =>
                {
                    await UniTask.Delay(TimeSpan.FromMilliseconds(1000));
                    firstCancel = true;
                });
            }
        }

        public void Menu(InputAction.CallbackContext context)
        {
            Debug.Log("Menu");
        }

        public void Up(InputAction.CallbackContext context)
        {
            Debug.Log("Up");
        }

        public void Down(InputAction.CallbackContext context)
        {
            Debug.Log("Down");
        }

        public void Right(InputAction.CallbackContext context)
        {
            Debug.Log("Right");
        }

        public void Left(InputAction.CallbackContext context)
        {
            Debug.Log("Left");
        }
    }
}