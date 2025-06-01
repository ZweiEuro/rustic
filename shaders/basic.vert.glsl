#version 400 core
layout (location = 0) in vec2 in_pos;
layout (location = 1) in vec2 uv_pos;

out vec2 texcoord; 


uniform mat4 camera_rotation;

void main()
{
    gl_Position = camera_rotation * vec4(in_pos, 0, 1);
    texcoord = uv_pos;
}
