use ndarray::{arr1, Array1, ArrayView1};

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

    pub fn l2_norm(x: ArrayView1<f64>) -> f64 {
        x.dot(&x).sqrt()
    }

    pub fn squared_length(x: ArrayView1<f64>) -> f64 {
        x.dot(&x)
    }

    pub fn normalize(x: Array1<f64>) -> Array1<f64> {
        // TODO Need to create Vec3 and use that instead in
        // here
        let mut vec3 = arr1(&[x[0], x[1], x[2]]);
        let norm: f64 = Vec4::l2_norm(vec3.view());
        vec3.mapv_inplace(|e| e / norm);
        arr1(&[vec3[0], vec3[1], vec3[2], x[3]])
    }

    pub fn cross(a: Array1<f64>, b: Array1<f64>) -> Array1<f64> {
        let mut c = arr1(&[0.0, 0.0, 0.0, 0.0]);
        c[0] = a[1] * b[2] - a[2] * b[1];
        c[1] = -a[0] * b[2] + a[2] * b[0];
        c[2] = a[0] * b[1] - a[1] * b[0];

        c
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
