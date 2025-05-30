#version 330 core
layout (location = 0) in vec2 in_pos;
layout (location = 1) in vec3 aColor;

out vec3 ourColor; 


void main()
{
    gl_Position = vec4(in_pos.x, in_pos.y, 0.0, 1.0);
    ourColor = aColor;
}
