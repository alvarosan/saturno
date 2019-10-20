/**
 * Dynamic LUT generator. This was contributed in github
 * by https://github.com/ajkavanagh/rust-mandelbrot/blob/master/src/main.rs
 */
pub fn rainbow(c: u32) -> Vec<image::Rgba<u8>> {
    (0..c)
        .map(|i| image::Rgba::<u8> {
            data: [
                sin_to_dec(c, i, 0.0 * std::f64::consts::PI * 2.0 / 3.0),
                sin_to_dec(c, i, 2.0 * std::f64::consts::PI * 2.0 / 3.0),
                sin_to_dec(c, i, 1.0 * std::f64::consts::PI * 2.0 / 3.0),
                255,
            ],
        })
        .collect()
}

fn sin_to_dec(c: u32, i: u32, phase: f64) -> u8 {
    let s =
        (std::f64::consts::PI / (c as f64) * 2.0 * (i as f64) + phase).sin();
    (((s * 127.0) + 128.0).floor()) as u8
}
