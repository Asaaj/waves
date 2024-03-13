#version 300 es

precision mediump float;

#define M_PI 3.1415926535898
#define M_TAU 2.0 * M_PI

in vec2 fragPosition;
in vec4 fragColor;

uniform vec2 u_viewportSize;

uniform mat2 u_oscillatorLocation;
uniform float u_wavelength;
uniform vec2 u_phase;

out vec4 outColor;

float amplitude_at(float distance, float wavelength, float phase) {
    return sin(M_TAU * distance / wavelength - phase);
}

float to01(float neg_1_to_pos_1) {
    return (neg_1_to_pos_1 + 1.0) / 2.0;
}

vec2 to01(vec2 neg_1_to_pos_1) {
    return vec2(to01(neg_1_to_pos_1.x), to01(neg_1_to_pos_1.y));
}

void main() {
    vec2 frag_pos = fragPosition;
    vec2 distance = vec2(length(frag_pos - u_oscillatorLocation[0]), length(frag_pos - u_oscillatorLocation[1]));

    vec2 amplitude = vec2(
        amplitude_at(distance.x, u_wavelength, u_phase.x),
        amplitude_at(distance.y, u_wavelength, u_phase.y)
    );
    float interference = amplitude.x + amplitude.y;

//    vec4 full_color = vec4(to01(amplitude), 0.0, to01(interference / 2.0));
    vec4 full_color = vec4(to01(amplitude), to01(interference / 2.0), 1.0);
    outColor = full_color * vec4(0.4, 0.4, 1.0, 1.0);
}
