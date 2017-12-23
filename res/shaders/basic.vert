#version 330 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 uv;

uniform mat4 wvp;

smooth out vec2 frag_uv;

void main() {
    gl_Position = wvp * vec4(position, 1);
    frag_uv = uv;
}
