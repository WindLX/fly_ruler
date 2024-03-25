using FlyRuler.Base;
using MarkupAttributes;
using UnityEngine;

namespace FlyRuler.Entity
{
    public class Fire : LinearUIBase
    {
        [Box("Light")]
        public Light fireLight;
        [TitleGroup("./Weight")]
        public float lightTop;
        public float lightBottom;

        [Box("Particle")]
        public ParticleSystem fireParticle;
        [TitleGroup("./LifeTime")]
        public float particleLifeTimeTop;
        public float particleLifeTimeBottom;
        [TitleGroup("./Speed")]
        public float particleSpeedTop;
        public float particleSpeedBottom;

        private float thrustTop;
        private float thrustBottom;

        public void Init(float thrustTop, float thrustBottom)
        {
            this.thrustTop = thrustTop;
            this.thrustBottom = thrustBottom;
        }

        protected override void ValueSetter(float value)
        {
            fireLight.intensity = (lightTop - lightBottom) * (value - thrustBottom) / (thrustTop - thrustBottom) + lightBottom;
            var fireParticleMain = fireParticle.main;
            fireParticleMain.startSpeed = (particleSpeedTop - particleSpeedBottom) * (value - thrustBottom) / (thrustTop - thrustBottom) + particleSpeedBottom;
            fireParticleMain.startLifetime = (particleLifeTimeTop - particleLifeTimeBottom) * (value - thrustBottom) / (thrustTop - thrustBottom) + particleLifeTimeBottom;
        }
    }
}