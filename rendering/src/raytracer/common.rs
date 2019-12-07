use ndarray::{arr1, Array1, ArrayView1};
use rand::Rng;

pub struct Vec4 {
    data: Array1<f64>,
}

impl Vec4 {
    pub fn x(&self) -> f64 {
        self.data[0]
    }
    pub fn y(&self) -> f64 {
        self.data[1]
    }
    pub fn z(&self) -> f64 {
        self.data[2]
    }
    pub fn w(&self) -> f64 {
        self.data[3]
    }
    pub fn r(&self) -> f64 {
        self.data[0]
    }
    pub fn g(&self) -> f64 {
        self.data[1]
    }
    pub fn b(&self) -> f64 {
        self.data[2]
    }
    pub fn a(&self) -> f64 {
        self.data[3]
    }

    pub fn normalized(&self) -> Vec4 {
        Vec4 {
            data: Vec4::normalize(self.data.clone()),
        }
    }

    fn l2_norm(x: ArrayView1<f64>) -> f64 {
        x.dot(&x).sqrt()
    }

    pub fn squared_length(x: ArrayView1<f64>) -> f64 {
        x.dot(&x)
    }

    pub fn normalize(mut x: Array1<f64>) -> Array1<f64> {
        // TODO Need to create Vec3 and use that instead in
        // here
        let mut vec3 = arr1(&[x[0], x[1], x[2]]);
        let norm: f64 = Vec4::l2_norm(vec3.view());
        vec3.mapv_inplace(|e| e / norm);
        arr1(&[vec3[0], vec3[1], vec3[2], x[3]])
    }

    // TODO This is semantically Vec3
    pub fn random(min: f64, max: f64) -> Array1<f64> {
        let mut rng = rand::thread_rng();
        arr1(&[rng.gen_range(min, max),
            rng.gen_range(min, max),
            rng.gen_range(min, max)])
    }
}

pub struct Ray {
    pub origin: Array1<f64>,
    pub direction: Array1<f64>,
}

impl Ray {
    pub fn new(origin: Array1<f64>, direction: Array1<f64>) -> Ray {
        Ray {
            origin,
            direction: Vec4::normalize(direction),
        }
    }

    pub fn point_at_parameter(&self, t: f64) -> Array1<f64> {
        self.origin.clone() + t * self.direction.clone()
    }
}
