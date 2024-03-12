#version 300 es

precision mediump float;

in vec2 fragPosition;
in vec4 fragColor;

uniform vec2 u_viewportSize;

uniform vec2 u_oscillatorLocation;
uniform float u_wavelength;
uniform float u_phase;

out vec4 outColor;

void main() {
    outColor = fragColor;
}
