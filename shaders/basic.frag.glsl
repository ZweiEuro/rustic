#version 440 core

in vec2 texcoord;

uniform sampler2D tex;


layout(location = 0) out vec4 diffuseColor;

void main()
{
    diffuseColor = texture(tex, texcoord); 
}
