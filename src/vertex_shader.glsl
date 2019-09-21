#version 140

in vec2 position;
out vec2 c;

uniform mat3 inv_view;

void main() {
    // Go from view to world space
    c = (inv_view * vec3(position, 1.0)).xy;
    gl_Position = vec4(position, 0.0, 1.0);
}