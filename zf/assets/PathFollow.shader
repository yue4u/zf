shader_type spatial;
render_mode unshaded, cull_disabled;

void fragment() {
	ALBEDO = vec3(.0);
	ALPHA = 0.;
	if (fract(TIME * 0.05 + UV.y * 10.) < .2){
		ALBEDO += vec3(.2, 0.2, 0.6);
		ALPHA += 0.05;
	}
	// if (fract(UV.y * 10.) < .1) {
	// 	ALBEDO += vec3(1.0, 1.0, 1.0);
	// 	ALPHA += 0.01;
	// }
}