shader_type canvas_item;

uniform sampler2D image; 
uniform vec4 color: source_color;

void vertex() {
	// Called for every vertex the material is visible on.
}

void fragment() {
	// Called for every pixel the material is visible on.
	vec4 actual_color = texture(image, UV);
	if (actual_color.a != 0.0) {
		actual_color.rgb = actual_color.rgb * color.rgb;
	}
	COLOR = actual_color;
}

void light() {
	// Called for every pixel for every light affecting the CanvasItem.
}
