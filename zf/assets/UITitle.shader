// adjusted from https://www.shadertoy.com/view/wld3WN
shader_type canvas_item;

float rand(vec2 co){
    return fract(sin(dot(co.xy ,vec2(12.9898,78.233))) * 43758.5453);
}

void fragment() {
    vec4 tex = texture(TEXTURE, UV);
    float t = TIME / 1000.;
    float r = rand(vec2(UV.y * 10., t));

    vec2 uvGlitch = UV;

    if (r < .01){
        uvGlitch.x -= r * 10.;
        uvGlitch.y -= r * 10.;

        tex = texture(TEXTURE, uvGlitch);
        tex.a = COLOR.a * tex.a;
        COLOR = tex;
    }else{
        COLOR = min(COLOR, tex);
    }
}