use crate::raytracer::actor::RayTraceable;
use crate::raytracer::actor::Sphere;
use crate::raytracer::camera::Camera;
use crate::raytracer::canvas::Canvas;
use crate::raytracer::common::Vec4;
use crate::raytracer::material::Dielectric;
use crate::raytracer::material::Lambertian;
use crate::raytracer::material::Metal;
use crate::raytracer::material::Shading;
use ndarray::arr1;
use rand::Rng;

pub fn random_book_cover() -> Vec<Box<dyn RayTraceable>> {
    let mut actors: Vec<Box<dyn RayTraceable>> = Vec::with_capacity(1000);

    // World orbe
    actors.push(Box::new(Sphere {
        center: arr1(&[0.0, -1000.0, 0.0, 1.0]),
        radius: 1000.0,
        material: Box::new(Lambertian::new(
            arr1(&[0.5, 0.5, 0.5, 1.0]),
            Shading::COLOR,
        )),
    }) as Box<dyn RayTraceable>);

    let mut rng = rand::thread_rng();
    let max = 10;
    for a in -max..max {
        for b in -max..max {
            let choose_mat: f64 = rng.gen();
            let center = arr1(&[
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>() as f64,
                1.0,
            ]);
            let radius = 0.2;

            // Filter colliding spheres
            let max_pos = arr1(&[4.0, 0.2, 0.0, 1.0]);
            if Vec4::l2_norm((center.clone() - max_pos.clone()).view()) > 2.0 {
                if choose_mat < 0.8 {
                    actors.push(Box::new(Sphere {
                        center,
                        radius,
                        material: Box::new(Lambertian::new(
                            arr1(&[
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                1.0,
                            ]),
                            Shading::COLOR,
                        )),
                    })
                        as Box<dyn RayTraceable>);
                } else if choose_mat < 0.95 {
                    actors.push(Box::new(Sphere {
                        center,
                        radius,
                        material: Box::new(Metal::new(
                            arr1(&[
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                1.0,
                            ]),
                            Shading::COLOR,
                            0.5 * rng.gen::<f64>(),
                        )),
                    })
                        as Box<dyn RayTraceable>);
                } else {
                    actors.push(Box::new(Sphere {
                        center,
                        radius,
                        material: Box::new(Dielectric::new(
                            arr1(&[1.0, 1.0, 1.0, 0.0]),
                            Shading::COLOR,
                            1.5,
                        )),
                    })
                        as Box<dyn RayTraceable>);
                }
            }
        }
    }

    // Center spheres
    actors.push(Box::new(Sphere {
        center: arr1(&[0.0, 1.0, 0.0, 1.0]),
        radius: 1.0,
        material: Box::new(Dielectric::new(
            arr1(&[1.0, 1.0, 1.0, 0.0]),
            Shading::COLOR,
            1.5,
        )),
    }) as Box<dyn RayTraceable>);

    actors.push(Box::new(Sphere {
        center: arr1(&[-4.0, 1.0, 0.0, 1.0]),
        radius: 1.0,
        material: Box::new(Lambertian::new(
            arr1(&[0.4, 0.2, 0.1, 1.0]),
            Shading::COLOR,
        )),
    }) as Box<dyn RayTraceable>);

    actors.push(Box::new(Sphere {
        center: arr1(&[4.0, 1.0, 0.0, 1.0]),
        radius: 1.0,
        material: Box::new(Metal::new(
            arr1(&[0.7, 0.6, 0.5, 1.0]),
            Shading::COLOR,
            0.0,
        )),
    }) as Box<dyn RayTraceable>);

    actors
}

pub fn two_spheres_normals() -> Vec<Box<dyn RayTraceable>> {
    let mut actors: Vec<Box<dyn RayTraceable>> = Vec::with_capacity(1000);
    actors.push(Box::new(Sphere {
        center: arr1(&[0.0, 0.0, -1.0, 1.0]),
        radius: 0.5,
        material: Box::new(Lambertian::new(
            arr1(&[1.0, 0.0, 0.0, 1.0]),
            Shading::NORMALS,
        )),
    }) as Box<dyn RayTraceable>);

    actors.push(Box::new(Sphere {
        center: arr1(&[0.0, -100.5, -1.0, 1.0]),
        radius: 100.0,
        material: Box::new(Lambertian::new(
            arr1(&[0.0, 1.0, 0.0, 1.0]),
            Shading::NORMALS,
        )),
    }) as Box<dyn RayTraceable>);

    actors
}


pub fn get_renderer(scene_id: u32) -> Box<Canvas> {
    let dims: [u32; 2] = [200, 133];
    let actors: Vec<Box<dyn RayTraceable>>;
    let camera: Camera;
    match scene_id {
        0 => {
            actors = random_book_cover();
            camera = Camera::new(
                20.0,
                dims[0],
                dims[1],
                arr1(&[13.0, 2.0, 3.0, 1.0]),
                arr1(&[0.0, 0.0, 0.0, 1.0]),
                arr1(&[0.0, 1.0, 0.0, 0.0]),
                0.2,
            );
        }
        _ => {
            actors = two_spheres_normals();
            camera = Camera::new(
                90.0,
                dims[0],
                dims[1],
                arr1(&[0.0, 0.0, 0.0, 1.0]),
                arr1(&[0.0, 0.0, -1.0, 1.0]),
                arr1(&[0.0, 1.0, 0.0, 0.0]),
                0.0,
            );
        }
    }

    Box::new(Canvas::new(dims[0], dims[1], actors, 2, camera))
}
