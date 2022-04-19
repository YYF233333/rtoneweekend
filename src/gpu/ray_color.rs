use std::f32::INFINITY;

use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::{unit, Color};
use arrayfire::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn ray_color(rays: Vec<Ray>, world: &impl Hittable, depth: i32) -> Vec<Color> {
    rays.into_par_iter()
        .map(|r| {
            // If we've exceeded the ray bounce limit, no more light is gathered.
            if depth <= 0 {
                if cfg!(debug_assertions) {
                    // in debug mode, when exceed the bounce limit, return RED for clearer visualization
                    return Color::new(1., 0., 0.);
                } else {
                    return Color::new(0., 0., 0.);
                }
            }

            let mut final_color = Color::new(1.0, 1.0, 1.0);
            let mut remain_depth = depth;
            let mut cur_ray = r;
            while let Some(rec) = world.hit(cur_ray, 0.001, INFINITY) {
                if let Some((attenuation, scattered)) = rec.material.clone().scatter(cur_ray, rec) {
                    final_color = final_color * attenuation;
                    cur_ray = scattered;
                    remain_depth -= 1;
                    // If we've exceeded the ray bounce limit, no more light is gathered.
                    if remain_depth <= 0 {
                        if cfg!(debug_assertions) {
                            // in debug mode, when exceed the bounce limit, return RED for clearer visualization
                            return final_color * Color::new(1., 0., 0.);
                        } else {
                            return final_color * Color::new(0., 0., 0.);
                        }
                    }
                } else {
                    return final_color * Color::default();
                }
            }

            let unit_direction = unit(cur_ray.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            final_color * ((1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0))
        })
        .collect()
}

#[allow(dead_code)]
pub fn ray_color_default(r: Ray, world: &impl Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        if cfg!(debug_assertions) {
            return Color::new(1., 0., 0.);
        } else {
            return Color::new(0., 0., 0.);
        }
    }
    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        if let Some((attenuation, scattered)) = rec.material.clone().scatter(r, rec) {
            return attenuation * ray_color_default(scattered, world, depth - 1);
        }
        return Color::default();
    }

    let unit_direction = unit(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}
/*
#[cfg(test)]
mod test {
    use rand::random;

    use super::*;
    use crate::{
        camera::Camera,
        scene::test_scene,
        vec3::{Point3, Vec3},
    };

    #[test]
    fn equal() {
        static aspect_ratio: f32 = 3.0 / 2.0;
        let world = test_scene();
        let lookfrom = Point3::new(13., 2., 3.);
        let lookat = Point3::new(0., 0., 0.);
        let vup = Vec3::new(0., 1., 0.);
        let dist_to_focus = 10.0;
        let aperture = 0.1;
        let cam = Camera::new(
            lookfrom,
            lookat,
            vup,
            20.,
            aspect_ratio,
            aperture,
            dist_to_focus,
        );
        let u = random::<f32>();
        let v = random::<f32>();
        let r = cam.get_ray(u, v);
        let c1 = ray_color(r, &world, 50);
        let c2 = ray_color_default(r, &world, 50);
        eprintln!("c1: {:?} c2: {:?}", c1, c2);
        //assert!(c1 == c2);
    }
}
*/
