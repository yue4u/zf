shader_type canvas_item;
const float PI = 3.1415926538;
uniform float value : hint_range(0, 1) = 1.;

void fragment()
{
    COLOR.a = 0.;
    float delta = .2;
    float count = 100.;
    float x = 1. - fract(count * UV.x + 3. * TIME);
    float g = 1.73205080757;
    if (UV.y > (1. - g * UV.x * count)){
        if (UV.y < g * x && (UV.y > g * x - delta)) {
            COLOR = UV.x < value
                ? vec4(0., 1.0, 0.93, 1. - UV.x)
                : vec4(1., 1.0, 1.00, 0.2);
        }
    }
}