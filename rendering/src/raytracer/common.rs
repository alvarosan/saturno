use ndarray::{Array1, ArrayView1};

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

    pub fn normalize(mut x: Array1<f64>) -> Array1<f64> {
        let norm: f64 = Vec4::l2_norm(x.view());
        x.mapv_inplace(|e| e / norm);
        x
    }
}


pub struct Ray {
    pub origin: Array1<f64>,
    pub direction: Array1<f64>,
}

impl Ray {
    pub fn point_at_parameter(&self, t: f64) -> Array1<f64> {
        self.origin.clone() + t * self.direction.clone()
    }
}
