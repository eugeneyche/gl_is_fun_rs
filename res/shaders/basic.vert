#version 330 core

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 uv;

smooth out vec2 frag_uv;

void main() {
    gl_Position = vec4(position, 0, 1);
    frag_uv = uv;
}
