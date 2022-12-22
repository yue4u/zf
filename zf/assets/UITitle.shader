// adjusted from https://www.shadertoy.com/view/wld3WN
shader_type canvas_item;

float rand(vec2 co){
    return fract(sin(dot(co.xy ,vec2(12.9898,78.233))) * 43758.5453);
}

void fragment() {
    vec4 tex = texture(TEXTURE, UV);
    float t = TIME;
    float r = rand(vec2(t, t));

    bool glitch = fract(TIME) < .2 && (UV.x > r && UV.x < r + .1 || UV.y > r && UV.y < r + 0.2);
    if(glitch) {

        vec2 uvGlitch = UV;
        uvGlitch.x -= r / 5.0;
        uvGlitch.y -= r / 5.0;

        tex = texture(TEXTURE, uvGlitch);
        tex = COLOR * tex.a;
        tex.r -= 2. * sin(10. * UV.x + TIME);
        tex.g -= 2. * sin(20. * UV.y + TIME);
        tex.b -= UV.x + UV.y;
        COLOR = tex;
    }

    COLOR = vec4(COLOR.rgb, tex.a * COLOR.a);
}