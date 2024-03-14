using Godot;
using System;

public partial class DirectionBox : Godot.Control
{
    [Export]
    public int Direction { get; set; }

    private ShaderMaterial material;

    // Called when the node enters the scene tree for the first time.
    public override void _Ready()
    {
        material = GetNode<TextureRect>("Direction").Material as ShaderMaterial;
    }

    // Called every frame. 'delta' is the elapsed time since the previous frame.
    public override void _Process(double delta)
	{
        material.SetShaderParameter("direction", Direction / (Mathf.Pi * 2) + (40 / 1366 * 2 - 1) * Mathf.Pi);
	}
}
