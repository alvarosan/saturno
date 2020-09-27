
/**
 * Rust does not yet support structs with generic variable-lenght arrays. So,
 * for now, only 4C (RGBA) supported.
 *
 * https://medium.com/@iBelieve/rust-structs-with-generic-variable-length-arrays-7490b68499ea
 */
#[derive(Clone, Copy)]
pub struct Pixel<T> {
    pub data: [T; 4]
}

#[derive(Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub chan: u32,
    pub data: Vec<Pixel<u8>>,
}

impl Image {
    pub fn new(width: u32, height: u32, chan: u32) -> Image {
        let size = width as usize * height as usize;
        let mut data: Vec<Pixel<u8>> = Vec::with_capacity(size);
        data.resize(size, Pixel { data: [0, 0, 0, 0] as [u8; 4] });
        Image {
            width,
            height,
            chan,
            data,
        }
    }

    pub fn size(&self) -> usize {
        self.data.len() as usize
    }

    pub fn pixel_coordinate(width: u32, index: usize) -> (u32, u32) {
        // index = y * stride + x
        let stride = width as usize;
        let y = index / stride;
        let x = index - y as usize * stride;

        (x as u32, y as u32)
    }

    pub fn get_pixel_coordinate(&self, index: usize) -> (u32, u32) {
        // index = y * stride + x
        let stride = self.width as usize;
        let y = index / stride;
        let x = index - y as usize * stride;

        (x as u32, y as u32)
    }

    pub fn get_value(&self, x: u32, y: u32, c: u32) -> u8 {
        let index = y * self.width + x;
        let pixel = &self.data[index as usize];

        pixel.data[c as usize]
    }

    pub fn set_pixel(&mut self, index: usize, color: [u8; 4]) {
        self.data[index] = Pixel { data: color };
    }

    pub fn as_flat_vec_u8(&self) -> Vec<u8> {
        self.data.iter().flat_map(|pixel| pixel.data.iter()).cloned().collect()
    }
}
