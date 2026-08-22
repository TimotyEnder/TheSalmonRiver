shader_type spatial;
render_mode cull_front, unshaded;

// Inverted-hull outline.
// Works with any camera projection (orthogonal or perspective) because the
// vertex offset is computed in clip space and sized against the viewport,
// so "thickness" is always measured in real screen pixels.

uniform vec4 outline_color : source_color = vec4(0.075, 0.05, 0.04, 1.0);
uniform float thickness : hint_range(0.0, 32.0) = 10.0; // pixels
uniform float depth_offset : hint_range(0.0, 1.0) = 0.02; // push hull back so it doesn't z-fight with the mesh

void vertex() {
	mat4 mvp = PROJECTION_MATRIX * MODELVIEW_MATRIX;

	vec4 clip_pos = mvp * vec4(VERTEX, 1.0);
	vec3 clip_normal = mat3(mvp) * NORMAL;

	// Offset direction in NDC space, converted to pixels via viewport size.
	vec2 ndc_offset = normalize(clip_normal.xy + 1e-6) * thickness / VIEWPORT_SIZE;
	clip_pos.xy += ndc_offset * clip_pos.w;
	clip_pos.z -= depth_offset * clip_pos.w;

	POSITION = clip_pos;
}

void fragment() {
	ALBEDO = outline_color.rgb;
}
