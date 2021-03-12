use crate::raytracer::common::Ray;
use crate::raytracer::common::AABB;
use crate::raytracer::material::Lambertian;
use crate::raytracer::material::Scattering;
use crate::raytracer::material::Shading;
use ndarray::{arr1, Array1};

pub struct Hit {
    pub t: f64,
    pub point: Array1<f64>,
    pub normal: Array1<f64>,
    pub material: Box<dyn Scattering>,
}

impl Hit {
    pub fn new() -> Hit {
        Hit {
            t: 0.0,
            point: arr1(&[0.0, 0.0, 0.0, 1.0]),
            normal: arr1(&[1.0, 1.0, 1.0, 0.0]),
            material: Box::new(Lambertian::new(
                arr1(&[0.0, 0.0, 1.0, 1.0]),
                Shading::COLOR,
            )),
        }
    }

    pub fn copy(hit: &Hit) -> Hit {
        Hit {
            t: hit.t,
            point: hit.point.clone(),
            normal: hit.normal.clone(),
            material: hit.material.clone(),
        }
    }
}

/**
 * Traits in rust are how interfaces are implemented. Depending on their
 * usage, they can be statically or dynamically dispatched.
 *
 * This trait defines the characteristics of a ray-Hittable object.
 */
pub trait Hittable {
    fn is_hit(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        record: &mut Hit,
    ) -> bool;

    // FIXME Removed from the trait, as HittableList now implements
    // Hittable. Compute normal needs to be part of a different trait
    // (e.g. Renderable ?).
    //fn compute_normal(&self, point_sphere: &Array1<f64>) -> Array1<f64>;

    /**
     * Returns whether there is an AABB defined (e.g. would not be defined
     * for infinite planes, for instance).
     */
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB>;
}

pub trait RayTraceable: Hittable + Sync {}

// -----------------------------------------------------------------------------
pub struct Sphere {
    pub center: Array1<f64>,
    pub radius: f64,
    pub material: Box<dyn Scattering>,
}

impl Sphere {
    pub fn new(
        center: Array1<f64>,
        radius: f64,
        material: Box<dyn Scattering>,
    ) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    /**
     * P - C = Radial Vector
     *
     * Note that the range of the normalized components of the unit normals
     * is [-1.0, 1.0].
     */
    fn compute_normal(&self, point_sphere: &Array1<f64>) -> Array1<f64> {
        let n = (point_sphere.clone() - self.center.clone()) / self.radius;
        n
    }
}

impl Hittable for Sphere {
    /**
     * Solving the sphere equation analitically, leads to real solutions
     * (hit front / back) or a complex solution (miss).
     *
     * vec{radius} = vec{Ray} - vec{Center}
     *           X = Y
     *   dot(X, X) = dot(Y, Y)
     *
     * Substitute Ray = Origin + t * Dir and solve for t ...
     *
     * t^2 dot(Dir, Dir) + 2*t*dot(Dir, Orig - Cent) +
     *      dot(Orig-Cent, Orig-Cent) = radius^2
     *
     */
    fn is_hit(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        record: &mut Hit,
    ) -> bool {
        let oc = ray.origin.clone() - self.center.clone();
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            // Solution (-) In range ?
            let t = (-b - discriminant.sqrt()) / (a);
            if t_min < t && t < t_max {
                record.t = t;
                record.point = ray.point_at_parameter(t);
                record.normal = self.compute_normal(&ray.point_at_parameter(t));
                record.material = self.material.clone();
                return true;
            }

            // Solution (+) In range ?
            let t = (-b + discriminant.sqrt()) / (a);
            if t_min < t && t < t_max {
                record.t = t;
                record.point = ray.point_at_parameter(t);
                record.normal = self.compute_normal(&ray.point_at_parameter(t));
                record.material = self.material.clone();
                return true;
            }
        }
        false
    }

    /**
     * p_max = center + Radius
     * p_min = center - Radius
     *
     */
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        let radius = arr1(&[self.radius, self.radius, self.radius, 0.0]);
        Some(AABB::new(
            self.center.clone() - radius.clone(),
            self.center.clone() + radius,
        ))
    }
}

impl RayTraceable for Sphere {}

// -----------------------------------------------------------------------------
pub struct HittableList {
    pub actors: Vec<Box<dyn RayTraceable>>,
}

impl HittableList {
    pub fn new(actors: Vec<Box<dyn RayTraceable>>) -> HittableList {
        HittableList { actors }
    }
}

impl Hittable for HittableList {
    /**
     * Traverse the vector of RayTraceable instances, and keep track
     * of the closest hit (e.g. closest to the camera hence, not
     * occluded). The closest (t), becomes the maximum depth t we
     * willing to accept as a hit in the following actors.
     */
    fn is_hit(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        record: &mut Hit,
    ) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut temp_record = Hit::new();

        for actor in self.actors.iter() {
            if actor.is_hit(&ray, t_min, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;

                // Dereferencing the borrow (e.g. pointer) to assign to
                // the mutable borrowed piece of memory
                *record = Hit::copy(&temp_record);
            }
        }

        hit_anything
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        None
    }
}

pub struct BVHNode {}

impl BVHNode {
    pub fn new() -> BVHNode {
        BVHNode {}
    }
}

impl Hittable for BVHNode {
    fn is_hit(
        &self,
        _ray: &Ray,
        _t_min: f64,
        _t_max: f64,
        _record: &mut Hit,
    ) -> bool {
        // TODO Implement actually hitting it
        true
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        None
    }
}


////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use crate::raytracer::material::Primary;

    #[test]
    fn aabb_sphere_origin() {
        let material =
            Box::new(Primary::new(arr1(&[1.0, 0.0, 0.0, 1.0]), Shading::COLOR));
        let sphere = Sphere::new(arr1(&[0.0, 0.0, 0.0, 1.0]), 1.0, material);
        let result_aabb = sphere.bounding_box(0.0, 0.0);

        match result_aabb {
            Some(aabb) => {
                let min: Array1<f64> = aabb.min();
                let max: Array1<f64> = aabb.max();

                let diff = min - arr1(&[-1.0, -1.0, -1.0, 1.0]);
                assert!(diff == arr1(&[0.0, 0.0, 0.0, 0.0]));

                let diff = max - arr1(&[1.0, 1.0, 1.0, 1.0]);
                assert!(diff == arr1(&[0.0, 0.0, 0.0, 0.0]));
            }
            None => assert!(false),
        }
    }

    #[test]
    fn aabb_sphere_shifted() {
        let material =
            Box::new(Primary::new(arr1(&[1.0, 0.0, 0.0, 1.0]), Shading::COLOR));
        let sphere = Sphere::new(arr1(&[1.0, 1.0, 1.0, 1.0]), 0.5, material);
        let result_aabb = sphere.bounding_box(0.0, 0.0);

        match result_aabb {
            Some(aabb) => {
                let min: Array1<f64> = aabb.min();
                let max: Array1<f64> = aabb.max();

                let diff = min - arr1(&[0.5, 0.5, 0.5, 1.0]);
                assert!(diff == arr1(&[0.0, 0.0, 0.0, 0.0]));

                let diff = max - arr1(&[1.5, 1.5, 1.5, 1.0]);
                assert!(diff == arr1(&[0.0, 0.0, 0.0, 0.0]));
            }
            None => assert!(false),
        }
    }
}
