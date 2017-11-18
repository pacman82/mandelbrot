#version 140

in vec2 c;
out vec4 color;

uniform int iter;

void main() {

    vec2 z = c;

    int i;
    for (i=iter; i != 0; i--){
        float real = z.x * z.x - z.y * z.y + c.x;
        float imag = 2.0 * z.x * z.y + c.y;

        // Sequences with abs(z) > 2 will always diverge
        if (real * real + imag * imag > 4.0)
            break;

        z.x = real;
        z.y = imag;
    }

    float conv = float(i) / float(iter);

    float red = i == 0 ? 0.0 : 1.0 - conv;
    float green = 0.0;
    float blue = i == 0 ? 0.0 : conv;

    color = vec4(red, green, blue, 1.0);
}