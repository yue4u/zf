shader_type spatial;
render_mode unshaded, cull_disabled;

void vertex() {
	VERTEX.y += sin(TIME + (VERTEX.x + VERTEX.y) * 2.) * 0.1;
}

void fragment() {
	ALBEDO = vec3(.0);
	ALPHA = 0.;
	EMISSION = vec3(1.);
	if (fract(TIME * 0.05 + (NORMAL.x + NORMAL.y) * 2.) < .2){
		ALBEDO += vec3(NORMAL.x, 2., NORMAL.z);
		ALPHA += max(.1, .5 + .5 * sin(TIME));
	}
}