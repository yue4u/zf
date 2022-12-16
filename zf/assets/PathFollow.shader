shader_type spatial;
render_mode unshaded, cull_disabled;

void fragment() {
	ALBEDO = vec3(.2, 0.2, 0.6);
	ALPHA = fract(UV.y * 10.) < .4 ? 0.05 : 0.;
}