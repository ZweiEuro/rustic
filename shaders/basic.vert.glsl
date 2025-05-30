#version 330 core
layout (location = 0) in vec2 in_pos;

out vec4 vertexColor; 

void main()
{
    gl_Position = vec4(in_pos.x, in_pos.y, 0.0, 1.0);
    vertexColor = vec4(1.0, 0.0, 0.0, 0.0);
}
