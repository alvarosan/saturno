use crate::raytracer::actor::Hit;
use crate::raytracer::common::Ray;
use crate::raytracer::common::Vec4;

use ndarray::{arr1, Array1};


fn random_dir_unit_sphere() -> Array1<f64> {
    let mut dir = arr1(&[std::f64::MAX, 0.0, 0.0]);

    while Vec4::squared_length(dir.view()) >= 1.0 {
        dir = Vec4::random(-1.0, 1.0);
    }

    arr1(&[dir[0], dir[1], dir[2], 0.0])
}

/**
 *  Specular reflection.
 *
 *  Re = In + 2 |In . N| N
 */
pub fn reflect(fuzz: f64, incident: &Ray, hit: &Hit) -> Ray {
    let dir = incident.direction.clone() -
        2.0 * incident.direction.dot(&hit.normal) * hit.normal.clone();

    Ray::new(hit.point.clone(), dir +
             fuzz * super::material::random_dir_unit_sphere())
}

#[derive(Clone)]
pub enum Shading {
    COLOR,
    NORMALS,
}

pub trait Scattering {
    fn scatter(
        &self,
        incident: &Ray,
        hit_record: &Hit,
        attenuation: &mut Array1<f64>,
        scattered: &mut Ray,
    ) -> bool;

    fn color(&self, hit: &Hit) -> Array1<f64>;

    fn clone_box(&self) -> Box<dyn Scattering>;
}

//https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/5
impl Clone for Box<dyn Scattering> {
    fn clone(&self) -> Box<dyn Scattering> {
        self.clone_box()
    }
}

// ----------------------------------------------------------------------------
/**
 * 0 Scattering material (only primary rays).
 *
 */

// Derives self.clone(), which is then used in the clone_box implementation.
#[derive(Clone)]
pub struct Primary {
    pub color: Array1<f64>,
    pub shading: Shading,
}

impl Primary {
    pub fn new(color: Array1<f64>, shading: Shading) -> Primary {
        Primary { color, shading }
    }
}

impl Scattering for Primary {
    fn scatter(
        &self,
        _incident: &Ray,
        hit_record: &Hit,
        attenuation: &mut Array1<f64>,
        _scattered: &mut Ray,
    ) -> bool {
        *attenuation = self.color(&hit_record);
        false
    }

    fn color(&self, hit: &Hit) -> Array1<f64> {
        match self.shading {
            Shading::COLOR => return self.color.clone(),
            Shading::NORMALS => {
                let normal = &hit.normal;

                // In order to use the normal vectors (i,j,k) as (r,g,b)
                // they need to be mapped from [-1.0, 1.0] to the
                // [0.0, 1.0] range.
                return (normal + 1.0) * 0.5;
            }
        }
    }

    fn clone_box(&self) -> Box<dyn Scattering> {
        Box::new((*self).clone())
    }
}

// ----------------------------------------------------------------------------
/**
 * Ideal diffusive material (Lambertian). Albedo is a measure of the
 * diffuse reflection ("whiteness")[0- black body, absorbs 100%, 1- ideal
 * reflector].
 */

// Derives self.clone(), which is then used in the clone_box implementation.
#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Array1<f64>,
    pub shading: Shading,
}

impl Lambertian {
    pub fn new(albedo: Array1<f64>, shading: Shading) -> Lambertian {
        Lambertian { albedo, shading }
    }
}

impl Scattering for Lambertian {
    fn scatter(
        &self,
        _incident: &Ray,
        hit_record: &Hit,
        attenuation: &mut Array1<f64>,
        scattered: &mut Ray,
    ) -> bool {
        let target = hit_record.point.clone()
            + hit_record.normal.clone()
            + random_dir_unit_sphere();

        *scattered = Ray::new(
            hit_record.point.clone(),
            target - hit_record.point.clone(),
        );

        *attenuation = self.color(&hit_record);

        true
    }

    // TODO Is it possible to share this trait definition among several
    // structs (?)
    fn color(&self, hit: &Hit) -> Array1<f64> {
        match self.shading {
            Shading::COLOR => return self.albedo.clone(),
            Shading::NORMALS => {
                let normal = &hit.normal;

                // In order to use the normal vectors (i,j,k) as (r,g,b)
                // they need to be mapped from [-1.0, 1.0] to the
                // [0.0, 1.0] range.
                return (normal + 1.0) * 0.5;
            }
        }
    }

    // TODO Is it possible to share this trait definition among several
    // structs (?)
    fn clone_box(&self) -> Box<dyn Scattering> {
        Box::new((*self).clone())
    }
}

// ----------------------------------------------------------------------------
/**
 * Metal (perfect reflector).
 *
 */

// Derives self.clone(), which is then used in the clone_box implementation.
#[derive(Clone)]
pub struct Metal {
    pub color: Array1<f64>,
    pub shading: Shading,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(color: Array1<f64>, shading: Shading, fuzz: f64) -> Metal {
        Metal { color, shading, fuzz }
    }

}

impl Scattering for Metal {
    fn scatter(
        &self,
        incident: &Ray,
        hit_record: &Hit,
        attenuation: &mut Array1<f64>,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = reflect(self.fuzz, &incident, &hit_record);
        *attenuation = self.color(&hit_record);
        scattered.direction.dot(&hit_record.normal) > 0.0
    }

    fn color(&self, hit: &Hit) -> Array1<f64> {
        match self.shading {
            Shading::COLOR => return self.color.clone(),
            Shading::NORMALS => {
                let normal = &hit.normal;

                // In order to use the normal vectors (i,j,k) as (r,g,b)
                // they need to be mapped from [-1.0, 1.0] to the
                // [0.0, 1.0] range.
                return (normal + 1.0) * 0.5;
            }
        }
    }

    fn clone_box(&self) -> Box<dyn Scattering> {
        Box::new((*self).clone())
    }
}

// ----------------------------------------------------------------------------
/**
 * Dielectric material.
 *
 * Refracts light following Snell's law and angle-varying reflectivity using
 * a polynomial approximation (Christophe Schlick).
 *
 */

#[derive(Clone)]
pub struct Dielectric {
    pub color: Array1<f64>,
    pub shading: Shading,
    pub refraction_idx: f64,
    pub refraction_idx_ext: f64,
}

//TODO Check qualitativley htat the normals look continuous in the
//back of the sphere
pub fn is_test_ray(ray: &Ray) -> bool {
        let test_dir = arr1(&[-0.6558966847430886, -0.09623168735882363, -0.7486915261248114, 0.0]);

        //let test_dir = arr1(&[0.0, 0.0, -1.0, 0.0]);
        test_dir == ray.direction
}

impl Dielectric {
    pub fn new(color: Array1<f64>, shading: Shading, refraction_idx: f64) -> Dielectric {
        // Air
        let refraction_idx_ext = 1.0;
        Dielectric { color, shading, refraction_idx, refraction_idx_ext }
    }

    /**
     * Snell's Law:
     * n_inci * sin(Theta_inci) = n_trans * sin(Theta_trans)
     *
     * Handles total internal reflection by checking the discriminant first.
     *
     * n_i ( Ray_i - Cos(Theta_i) Norm ) = n_t ( Ray_t + Cos(Theta_t) Norm )
     */
    pub fn refract(&self, incident: &Ray, normal: Array1<f64>, hit: &Hit, ni_over_nt: f64, refracted: &mut Ray) -> bool {
        // TODO!!!! the dot products might be  applying the 0s too!!!
        // this is the problem of using the whole vector... (homogeneous)
        //let ri_dot_normal = incident.direction.dot(&normal);
        //println!(">>> slice {}", incident.direction.slice(s![0..3]));
        let ri_dot_normal = incident.direction.slice(s![0..3]).dot(&normal.slice(s![0..3]));

        //println!(">>> norms {} , {}", incident.direction.dot(&incident.direction).sqrt(), normal.dot(&normal).sqrt());

        // Discriminant
        let sq_cos_theta_t = 1.0 -
            ni_over_nt.powi(2) * (1.0 - ri_dot_normal.powi(2));

        if is_test_ray(incident) {
            println!(">>>! {} {} {} {} {}", ri_dot_normal, ri_dot_normal.powi(2), ni_over_nt, ni_over_nt.powi(2), sq_cos_theta_t);
            // println!(">>> sqcostheta_t {}", sq_cos_theta_t);
            
        }
            if is_test_ray(incident) {
                let sinthetasq = -ri_dot_normal.powi(2) + 1.0;
                let mut angle = 666.666;
                if sinthetasq > 0.0 {
                    angle = sinthetasq.sqrt().asin() * 180.0 / std::f64::consts::PI;
                }
                let anglecos = ri_dot_normal.acos() * 180.0 / std::f64::consts::PI;
                println!(">>> angle ri_normal (sin),  (cos): {} , {}", angle, anglecos);

            }

        if sq_cos_theta_t > 0.0 {
            //println!(">>> point: {}, dir: {}", hit.point, dir);
            let dir =
                ni_over_nt * (incident.direction.clone() - normal.clone() * ri_dot_normal) -
                normal * sq_cos_theta_t.sqrt();

            *refracted = Ray::new(hit.point.clone(), dir);

            return true;
        }

        //println!(">>> sq_cos_theta_t: {}", sq_cos_theta_t);
        false
    }
}

impl Scattering for Dielectric {
    fn scatter(
        &self,
        incident: &Ray,
        hit_record: &Hit,
        attenuation: &mut Array1<f64>,
        scattered: &mut Ray,
    ) -> bool {

        let mut outward_normal = hit_record.normal.clone();
        let mut ni_over_nt = self.refraction_idx_ext / self.refraction_idx;

        // Change signs and invert refraction ratio if the normal points 
        // inwards (default outwards; but when the ray exits then it needs
        // to be inverted).
        if hit_record.normal.dot(&incident.direction) > 0.0 {
            if is_test_ray(incident) {
                println!(">>> definitely exiting");
            }

            outward_normal = -hit_record.normal.clone();
            ni_over_nt = self.refraction_idx / self.refraction_idx_ext;
        }

        *attenuation = self.color(&hit_record);
        let myscat = Ray::new(scattered.origin.clone(), scattered.direction.clone());
        if self.refract(&incident, outward_normal, hit_record, ni_over_nt,
                        scattered) {
            return true;
        }
        else {
            *scattered = reflect(0.0, &incident, hit_record);
            if is_test_ray(incident) {
                println!(">>> refract false!");
            }
            //println!(">>> incident: {}", incident.direction);
            //panic!("this is a terrible mistake!");
            return  false;
        }



    }

    fn color(&self, hit: &Hit) -> Array1<f64> {
        match self.shading {
            Shading::COLOR => return self.color.clone(),
            Shading::NORMALS => {
                let normal = &hit.normal;

                // In order to use the normal vectors (i,j,k) as (r,g,b)
                // they need to be mapped from [-1.0, 1.0] to the
                // [0.0, 1.0] range.
                return (normal + 1.0) * 0.5;
            }
        }
    }

    fn clone_box(&self) -> Box<dyn Scattering> {
        Box::new((*self).clone())
    }
}
