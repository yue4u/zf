shader_type canvas_item;

const float PI = 3.1415926538;
const float delta = .2;
const float count = 100.;
const float g = 1.73205080757;

uniform float value : hint_range(0, 1) = 1.;
uniform float prev_value : hint_range(0, 1) = 1.;

void fragment()
{
    COLOR.a = 0.;
    float x = 1. - fract(count * UV.x + 3. * TIME);
    if (UV.y > (1. - g * UV.x * count)) {
        if (UV.y < g * x && (UV.y > g * x - delta)) {
            vec3 c = value > .6
                ? vec3(0., 1.0, 0.93) // cyan
                : (value > .3
                    ? vec3(1., 0.61, 0.) // orange
                    : vec3(1., 0.0, 0.42) // red
                  );
            COLOR = UV.x < value
                ? vec4(c, 1. - UV.x)
                : (abs(UV.x - prev_value) < 0.01
                    ? vec4(1., 1., 1., 1.) // high light
                    : vec4(1., 1.0, 1.00, 0.2) // dim grey
                  );
        }
    }
}