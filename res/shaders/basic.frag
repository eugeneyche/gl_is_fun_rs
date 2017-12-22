#version 330 core

smooth in vec2 frag_uv;

uniform sampler2D tex;

out vec4 fs_out;

void main() {
    fs_out = texture(tex, frag_uv);
}
