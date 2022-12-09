shader_type canvas_item;

uniform float factor : hint_range(0, 1) = 0.;

void fragment() {
    float f = max(1. - factor, 0.0001);
    vec2 screenSize = 1. / SCREEN_PIXEL_SIZE;
    vec2 pixelSize = 100. * SCREEN_PIXEL_SIZE / f;
    vec2 uv = floor(SCREEN_UV * screenSize / pixelSize) / screenSize * pixelSize;
	COLOR = texture(SCREEN_TEXTURE, uv);
}