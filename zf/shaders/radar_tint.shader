shader_type canvas_item;

void fragment() 
{
	COLOR = texture(TEXTURE, UV) * vec4(.6, .1, .1, 1.);
}