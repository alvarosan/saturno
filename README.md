### Build
```
cargo build --release
```

### Run
```
./target/release/mandelbrot --iter=50      \
                            --width=1024   \
                            --height=768   \
                            --x0=-2.0      \
                            --x1=1.0       \
                            --y0=-1.2      \
                            --y1=1.2 &&    \
                            xdg-open fractal.png
```
