shader_type spatial;
render_mode unshaded, cull_disabled;

void fragment() {
	ALBEDO = vec3(.0);
	ALPHA = 0.;
	if (fract(TIME * 0.05 + UV.y * 10.) < .2){
		ALBEDO += vec3(.2, 0.2, 0.6);
		if (fract((-UV.x + TIME) / 5.) < min(.5, abs(sin(TIME / 4.)))){
			ALBEDO = 1. - NORMAL;
			ALPHA = 1.
		}
		ALPHA += 0.05;
	}
}