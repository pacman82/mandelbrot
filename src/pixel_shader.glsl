#version 140

in vec2 c;
out vec4 color;

void main() {

    vec2 z = c;

    int i;
    for (i=500; i != 0; i--){
        float real = z.x * z.x - z.y * z.y + c.x;
        float imag = 2.0 * z.x * z.y + c.y;

        if (real * real + imag * imag > 4.0)
            break;

        z.x = real;
        z.y = imag;
    }

    float red = i == 0 ? 0.0 : 1.0 - float(i) / 500.0;
    float green = 0.0;
    float blue = i == 0 ? 0.0 : float(i) / 500.0;

    color = vec4(red, green, blue, 1.0);
}