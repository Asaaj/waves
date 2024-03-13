#version 300 es

precision mediump float;

in vec2 fragPosition;
in vec4 fragColor;

uniform sampler2D s_texture;

out vec4 outColor;

void main() {
    vec2 texture_position = (fragPosition + 1.0) / 2.0;
    outColor = mix(texture(s_texture, texture_position), fragColor, 0.0);
}
