#version 300 es

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec4 color;

out vec3 fragPosition;
out vec4 fragColor;

void main() {
    vec2 remapped = position.xy * 2.0 - 1.0;  // Using 0-1 coordinates instead of -1-1
    gl_Position = vec4(remapped, 0.0, 1.0);
    fragPosition = position;
    fragColor = color;
}