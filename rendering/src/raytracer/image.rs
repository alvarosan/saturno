/**
 * Rust does not yet support structs with generic variable-lenght arrays. So,
 * for now, only 4C (RGBA) supported.
 *
 * https://medium.com/@iBelieve/rust-structs-with-generic-variable-length-arrays-7490b68499ea
 */
#[derive(Clone, Copy)]
pub struct Pixel<T> {
    pub data: [T; 4],
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
        let data = Image::create_buffer(width, height);
        Image {
            width,
            height,
            chan,
            data,
        }
    }
    
    pub fn from_vec(width: u32, height: u32, chan: u32, source: Vec<u8>) -> Image {

        let mut data = Image::create_buffer(width, height);
        let channels: usize = 4;
        let mut index: usize = 0;

        // TODO Consider rewriting this with chained .filter(*).map(*)
        loop {
            let linear_index = channels * index;
            data[index] = Pixel {
                data: [source[linear_index], source[linear_index+1], source[linear_index+2],
                source[linear_index+3]]
            };
            
            index += 1;
            if index >= data.len() {
                break;
            }
        }

        Image {
            width,
            height,
            chan,
            data,
        }
    }

    fn create_buffer(width: u32, height: u32) -> Vec<Pixel<u8>> {
        let size = width as usize * height as usize;
        let mut data: Vec<Pixel<u8>> = Vec::with_capacity(size);
        data.resize(
            size,
            Pixel {
                data: [0, 0, 0, 0] as [u8; 4],
            },
        );

        data
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
        self.data
            .iter()
            .flat_map(|pixel| pixel.data.iter())
            .cloned()
            .collect()
    }
}



pub fn dimensions_equal(first: &Image, second: &Image) -> Result<bool, String> {
    if first.width != second.width ||
       first.height != second.height {

        let mut message = String::from("Dimension mismatch: ");
        message.push_str(second.width.to_string().as_str());
        message.push(',');
        message.push_str(second.height.to_string().as_str());

        return Err(message);
    }

    Ok(true)
}

/**
 * Based on vtkImageDifference
 *
 * Thresholds the color value by T and clamps the minimum to 0. Computes an error and
 * returns the thresholded RGB pixel value and the error.
 *
 * Reference:
 * * https://gitlab.kitware.com/vtk/vtk/-/blob/master/Imaging/Core/vtkImageDifference.cxx
 */
pub fn compute_thresholded_rgb(rgb: [u8; 3], threshold: f32) -> (Vec<f32>, f32) {
    let t_rgb: Vec<f32> = rgb.iter().map(
        |x| (*x as f32 - threshold).max(0.0)
        ).collect::<Vec<_>>();

    let error = t_rgb.clone().iter().sum::<f32>() / (3.0 * 255.0);

    return (t_rgb, error)
}

pub fn compute_buffer_difference(first: &Image, second: &Image, threshold: f32) -> Result<(Vec<u8>, f32), String> {

    match dimensions_equal(first, second) {
        Ok(_) => (),
        Err(message) => return Err(message)
    }

    let buff_a = first.as_flat_vec_u8();
    let buff_b = second.as_flat_vec_u8();
    let mut diff = vec![0; buff_a.len()];

    let channels: usize = 4;
    let mut index: usize = 0;
    let mut error: f32 = 0.0;

    loop {
        let r = ((buff_a[index] as i32 - buff_b[index] as i32).abs() * 2)  as u8;
        let g = ((buff_a[index+1] as i32 - buff_b[index+1] as i32).abs())  as u8;
        let b = ((buff_a[index+2] as i32 - buff_b[index+2] as i32).abs())  as u8;
        let a: u8 = 255;

        let t_rgb_error = compute_thresholded_rgb([r,g,b], threshold);
        error += t_rgb_error.1;

        diff[index] = t_rgb_error.0[0] as u8;
        diff[index+1] = t_rgb_error.0[1] as u8;
        diff[index+2] = t_rgb_error.0[2] as u8;
        diff[index+3] = a;
        
        index += channels;
        if index >= buff_a.len() {
            break;
        }
    }

    Ok((diff, error))
}
