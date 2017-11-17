#version 140

in vec2 position;
out vec2 c;

void main() {
    c = position;
    gl_Position = vec4(position, 0.0, 1.0);
}