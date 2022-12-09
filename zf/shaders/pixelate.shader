// https://www.youtube.com/watch?v=zgok9i8lyVo
shader_type canvas_item;

uniform float factor : hint_range(0, 1) = 1;

void fragment() {
    vec2 size_after = vec2(textureSize(TEXTURE, 0)) * factor;
    vec2 uv_after = round(UV * size_after) / size_after;
    COLOR = texture(TEXTURE, uv_after);
}