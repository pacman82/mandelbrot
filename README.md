# Mandelbrot

As the name suggests this application draws a Mandelbrot fractal. This means it interprets each
position of a pixel as a complex number `c` and a assigns the pixel a color depending on how fast
the sequence `z(n + 1) = z(n) * z(n) + c` diverges. The application utilizes the GPU through OpenGL
and should run reasonably fast.

## Build

This Application is written in Rust. So to build it you will need cargo. If you do not have cargo
installed yet, you can get it [here](https://rustup.rs).

If you have cargo installed, just check this repository out and run:

```bash
cargo run --release
```

To see a Mandelbrot fractal.