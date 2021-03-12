use ndarray::{arr1, Array1, ArrayView1};
use std::mem;

pub struct Vec4 {
    data: Array1<f64>,
}

impl Vec4 {
    pub fn new(data: Array1<f64>) -> Vec4 {
        Vec4 { data }
    }

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

    pub fn data(self) -> Array1<f64> {
        self.data
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

pub struct AABB {
    pub min: Array1<f64>,
    pub max: Array1<f64>,
}

impl AABB {
    pub fn new(min: Array1<f64>, max: Array1<f64>) -> AABB {
        AABB { min, max }
    }

    pub fn min(&self) -> Array1<f64> {
        self.min.clone()
    }

    pub fn max(&self) -> Array1<f64> {
        self.max.clone()
    }

    pub fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> bool {
        for index in 0..3 {
            let inv_dist: f64 = 1.0 / ray.direction[index];
            let mut t0: f64 = (self.min[index] - ray.origin[index]) * inv_dist;
            let mut t1: f64 = (self.max[index] - ray.origin[index]) * inv_dist;

            if inv_dist < 0.0 {
                mem::swap(&mut t0, &mut t1);
            }

            let tmin: f64 = if t0 > tmin { t0 } else { tmin };
            let tmax: f64 = if t1 < tmax { t1 } else { tmax };

            if tmax <= tmin {
                return false;
            }
        }

        true
    }
}


////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_at_parameter() {
        let ray =
            Ray::new(arr1(&[0.5, 0.6, 0.7, 1.0]), arr1(&[1.0, 1.0, 1.0, 0.0]));

        assert_eq!(ray.origin[2], 0.7);

        let point = ray.point_at_parameter(3.0);

        assert_eq!(point[0], 3.5);
        assert_eq!(point[1], 3.6);
        assert_eq!(point[2], 3.7);
    }

    #[test]
    fn aabb_hit() {
        let aabb = AABB::new(arr1(&[0.0, 0.0, 0.0]), arr1(&[1.0, 1.0, 1.0]));

        let negz = Ray::new(
            arr1(&[0.0, 0.0, -1.0, 1.0]),
            arr1(&[0.0, 0.0, -1.0, 0.0]),
        );
        assert!(!aabb.hit(&negz, 0.0, 10.0));

        let short = Ray::new(
            arr1(&[0.0, 0.0, -1.0, 1.0]),
            arr1(&[0.0, 0.0, -0.5, 0.0]),
        );
        assert!(!aabb.hit(&short, 0.0, 10.0));

        let tangent_mid =
            Ray::new(arr1(&[0.0, 0.0, -1.0, 1.0]), arr1(&[0.0, 0.0, 0.5, 0.0]));
        assert!(aabb.hit(&tangent_mid, 0.0, 10.0));

        let tangent_cross =
            Ray::new(arr1(&[0.0, 0.0, -1.0, 1.0]), arr1(&[0.0, 0.0, 1.5, 0.0]));
        assert!(aabb.hit(&tangent_cross, 0.0, 10.0));

        let cross =
            Ray::new(arr1(&[0.0, 0.0, -1.0, 1.0]), arr1(&[1.5, 1.5, 1.0, 0.0]));
        assert!(aabb.hit(&cross, 0.0, 10.0));

        let side =
            Ray::new(arr1(&[0.0, 0.0, -1.0, 1.0]), arr1(&[1.5, 1.5, 0.0, 0.0]));
        assert!(!aabb.hit(&side, 0.0, 10.0));

        let negz_cross =
            Ray::new(arr1(&[0.0, 0.0, 2.0, 1.0]), arr1(&[1.0, 0.5, -2.0, 0.0]));
        assert!(aabb.hit(&negz_cross, 0.0, 10.0));
    }
}
