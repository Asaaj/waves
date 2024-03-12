#version 300 es

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec4 color;

out vec2 fragPosition;
out vec4 fragColor;

void main() {
    gl_Position = vec4(position.xy, 0.0, 1.0);
    fragPosition = position.xy;
    fragColor = color;
}