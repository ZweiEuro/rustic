#version 100
attribute vec2 in_pos;
attribute vec2 in_uv;

uniform vec2 offset;

varying lowp vec2 texcoord;

void main() {
    gl_Position = vec4(in_pos + offset, 0, 1);
    texcoord = in_uv;
}
