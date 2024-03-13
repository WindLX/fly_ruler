shader_type canvas_item;

uniform float direction: hint_range(0.0, 1.0);

void vertex() {
	// Called for every vertex the material is visible on.
}

void fragment() {
	// Called for every pixel the material is visible on.
	vec2 uv = vec2(UV.x + direction, UV.y);
	COLOR = texture(TEXTURE, uv);
}

void light() {
	// Called for every pixel for every light affecting the CanvasItem.
}
