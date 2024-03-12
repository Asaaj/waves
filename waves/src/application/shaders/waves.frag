#version 300 es

precision mediump float;

#define M_PI 3.1415926535898

in vec2 fragPosition;
in vec4 fragColor;

uniform vec2 u_viewportSize;

uniform vec2 u_oscillatorLocation;
uniform float u_wavelength;
uniform float u_phase;

out vec4 outColor;

void main() {
    vec2 frag_pos = fragPosition;
    float distance = length(frag_pos - u_oscillatorLocation);

    float amplitude = sin((distance - u_phase) * 2.0 * M_PI / u_wavelength);
    float remapped_amplitude = (amplitude + 1.0) / 2.0;
    outColor = vec4(remapped_amplitude, 0.0, 0.0, 1.0);
}
