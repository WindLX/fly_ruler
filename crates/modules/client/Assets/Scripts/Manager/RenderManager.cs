using UnityEngine;
using UnityEngine.Rendering;
using FlyRuler.Base;
using System.IO;
using Tomlyn;
using System;
using Tomlyn.Model;

namespace FlyRuler.Manager
{
    public class RenderManager : SingletonMono<RenderManager>
    {
        public RenderPipelineAsset balancedRenderPipelineAsset;
        public RenderPipelineAsset highfidelityRenderPipelineAsset;
        public RenderPipelineAsset performantRenderPipelineAsset;
        public RenderPipelineAsset ultraRenderPipelineAsset;

        protected override void Awake()
        {
            string configFileUrl = Application.streamingAssetsPath + "/config.toml";
            if (File.Exists(configFileUrl))
            {
                using StreamReader sr = new(configFileUrl);
                var configStr = sr.ReadToEnd();
                var config = Toml.ToModel(configStr);
                try
                {
                    var fixed_time = (double)((TomlTable)config["system"])["fixed_time"];
                    Time.fixedDeltaTime = (float)fixed_time;
                    Debug.Log(Time.fixedDeltaTime);

                    var preset = (string)((TomlTable)config["render"])["preset"];
                    var pipelineAsset = preset switch
                    {
                        "Performant" => performantRenderPipelineAsset,
                        "Balanced" => balancedRenderPipelineAsset,
                        "Highfidelity" => highfidelityRenderPipelineAsset,
                        "Ultra" => ultraRenderPipelineAsset,
                        _ => highfidelityRenderPipelineAsset,
                    };
                    GraphicsSettings.defaultRenderPipeline = pipelineAsset;
                }
                catch (Exception ex)
                {
                    Debug.Log(ex);
                    GraphicsSettings.defaultRenderPipeline = highfidelityRenderPipelineAsset;
                }
            }
        }
    }
}