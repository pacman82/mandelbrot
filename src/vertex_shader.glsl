#version 140

in vec2 position;
out vec2 c;

uniform vec2 center;

void main() {
    c = center + position;
    gl_Position = vec4(position, 0.0, 1.0);
}