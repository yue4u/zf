shader_type spatial;
render_mode unshaded, cull_disabled;

void fragment() {
	ALBEDO = vec3(.0);
	ALPHA = 0.;
	float l = fract(TIME * 0.05 + UV.y * 10.);
	float w = .2;
	if (l < w){
		float border = .01;
		if (l < border || l > w - border){
			ALPHA = .5
		}
		ALBEDO += vec3(.2, 0.2, 0.6);

		if (fract((-UV.x + TIME) / 4.) < .01){
			ALPHA = 1.
		}
		ALPHA += 0.05;
	}
}