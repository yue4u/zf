shader_type spatial;
render_mode unshaded, cull_disabled;

uniform float hit: hint_range(0, 1) = 0.;

const float PI = 3.1415926;
const uint UINT_MAX = uint(0xffffffff);

const uvec3 k = uvec3(uint(0x456789ab), uint(0x6798ab45), uint(0x89ab4567));
const uvec3 u = uvec3(1, 2, 3);

void vertex() {
    float delta = 5. * TIME + (TIME * .05 + VERTEX.x + VERTEX.y) * 10.;
	VERTEX.y += sin(delta) * 0.1;
	VERTEX.x += fract(delta / 5.) * 0.1;
}

uvec2 uhash22(uvec2 n) {
    n ^= (n.yx << u.xy);
    n ^= (n.yx >> u.xy);
    n *= k.xy;
    n ^= (n.yx << u.xy);
    return n * k.xy;
}

float gtable2(vec2 lattice, vec2 p) {
    uvec2 n = floatBitsToUint(lattice);
    uint ind = uhash22(n).x >> uint(29);
    float _u = 0.92387953 * (ind < 4u ? p.x : p.y);
    float _v = 0.38268343 * (ind < 4u ? p.y : p.x);
    return ((ind & 1u) == 0u ? _u : -_u) + ((ind & 2u) == 0u ? _v: -_v);
}

float pnoise21(vec2 p) {
    vec2 n = floor(p);
    vec2 f = fract(p);
    float v[4];
    for(int j = 0; j < 2; j ++ ) {
        for(int i = 0; i < 2; i ++ ) {
            v[i + 2*j] = gtable2(
                n + vec2(float(i), float(j)),
                f - vec2(float(i), float(j))
            );
        }
    }
    f = f * f * f * (10.0 - 15.0 * f + 6.0 * f * f);
    return 0.5 * mix(
        mix(v[0], v[1], f[0]),
        mix(v[2], v[3], f[0]),
        f[1]
    ) + 0.5;
}

float base21(vec2 p) {
    return pnoise21(p) - .5;
}

float fbm21(vec2 p, float g) {
    float val = 0.0;
    float amp = 1.0;
    float freq = 1.0;
    for (int i =0; i < 4; i++){
        val += amp * base21(freq * p);
        amp *= g;
        freq *= 2.01;
    }
    return .5 * val + .5;
}

float base212(vec2 p) {
    return fbm21(p, .5);
}

float warp213(vec2 p, float g) {
    float val = 0.0;
    for (int i =0; i < 4; i++){
        val = base212(p + g * vec2(
            cos(2.0 * PI * val),
            sin(2.0 * PI * val)
        ));
    }
    return val;
}

vec4 tex(vec2 pos) {
    return vec4(
        vec3(warp213(pos, .1)),
        1.0
    );
}

vec3 hsv2rgb(vec3 c) {
    vec3 rgb = clamp(
        abs(
            mod(
                c.x * 6.0 + vec3(0.0, 4.0, 2.0),
                6.0
            ) - 3.0
        ) - 1.0,
        0.0,
        1.0
    );
    return c.z * mix(vec3(1.0), rgb, c.y);
}

void fragment() {
	ALBEDO = vec3(.0);
	EMISSION = vec3(1.);
	vec4 c = tex(VERTEX.xy + sin(TIME));
	ALBEDO = hsv2rgb(c.xyz + 0.01 * VERTEX) * 4. + vec3(0.,-1.5,-1.5);
    ALPHA = .2;
    if (fract(-fract(TIME * 5.) + (NORMAL.x / NORMAL.y) * 2.) < .5){
        ALPHA = VERTEX.z * .001;
	}
}