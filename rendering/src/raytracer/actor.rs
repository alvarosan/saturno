use crate::raytracer::common::Ray;
use crate::raytracer::common::Vec4;
use ndarray::Array1;

pub enum Shading {
    COLOR,
    NORMALS,
}

pub struct Hit {
    pub t: f64,
    pub point: Array1<f64>,
    pub normal: Array1<f64>,
}


/**
 * Traits in rust are how interfaces are implemented. Depending on their
 * usage, they can be statically or dinamically dispatched.
 */
pub trait Renderable {
    fn render(&self, hit: &Hit) -> Array1<f64>;
}

pub trait Hittable {
    fn is_hit(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        record: &mut Hit,
    ) -> bool;
    fn compute_normal(&self, point_sphere: &Array1<f64>) -> Array1<f64>;
}

pub trait RayTraceable: Renderable + Hittable {}

// -----------------------------------------------------------------------------
pub struct Sphere {
    pub center: Array1<f64>,
    pub radius: f64,
    pub color: Array1<f64>,
    pub shading: Shading,
}

impl Sphere {}

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
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            // Solution (-) In range ?
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t_min < t && t < t_max {
                record.t = t;
                record.point = ray.point_at_parameter(t);
                record.normal = self.compute_normal(&ray.point_at_parameter(t));
                return true;
            }

            // Solution (+) In range ?
            let t = (-b + discriminant.sqrt()) / (2.0 * a);
            if t_min < t && t < t_max {
                record.t = t;
                record.point = ray.point_at_parameter(t);
                record.normal = self.compute_normal(&ray.point_at_parameter(t));
                return true;
            }
        }
        false
    }

    /**
     * P - C = Radial Vector
     *
     * Note that the range of the normalized components of the unit normals
     * is [-1.0, 1.0].
     */
    fn compute_normal(&self, point_sphere: &Array1<f64>) -> Array1<f64> {
        let n = point_sphere.clone() - self.center.clone();
        Vec4::normalize(n)
    }
}

impl Renderable for Sphere {
    fn render(&self, hit: &Hit) -> Array1<f64> {
        match self.shading {
            Shading::COLOR => return self.color.clone(),
            Shading::NORMALS => {
                let normal = &hit.normal;

                // In order to use the normal vectors (i,j,k) as (r,g,b)
                // they need to be mapped from [-1.0, 1.0] to the
                // [0.0, 1.0] range.
                return 255.0 * ((normal + 1.0) * 0.5);
            }
        }
    }
}

impl RayTraceable for Sphere {}
