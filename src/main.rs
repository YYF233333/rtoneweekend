use std::{f32::INFINITY, io::Write, sync::Arc};

use hittable::Hittable;
use hittable_list::HittableList;
use material::*;
use rand::random;
use ray::Ray;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rtweekend::random_float;
use sphere::Sphere;
use vec3::{unit, Color, Point3};

use crate::{color::write_color, vec3::Vec3};
use camera::*;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;
fn ray_color(r: Ray, world: &impl Hittable, depth: i32) -> Color {
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
            return attenuation * ray_color(scattered, world, depth - 1);
        }
        return Color::default();
    }

    let unit_direction = unit(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f32>();
            let center = Point3::new(
                a as f32 + 0.9 * random::<f32>(),
                0.2,
                b as f32 + 0.9 * random::<f32>(),
            );

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_in_range(0.5, 1.);
                    let fuzz = random_float(0., 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.0,
        material3,
    )));

    world
}

#[allow(non_upper_case_globals)]
fn main() {
    // Image
    static aspect_ratio: f32 = 3.0 / 2.0;
    static image_width: i32 = 1200;
    static image_height: i32 = (image_width as f32 / aspect_ratio) as i32;
    static samples_per_pixel: i32 = 500;
    static max_depth: i32 = 50;

    // World

    let world = random_scene();

    // Camera

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

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        std::io::stderr().flush().unwrap();
        for i in 0..image_width {
            let pixel_color = (0..samples_per_pixel)
                .into_iter()
                .map(|_| {
                    let u = (i as f32 + random::<f32>()) / (image_width - 1) as f32;
                    let v = (j as f32 + random::<f32>()) / (image_height - 1) as f32;
                    let r = cam.get_ray(u, v);
                    ray_color(r, &world, max_depth)
                })
                .reduce(|acc, x| acc + x)
                .unwrap();
            write_color(&mut std::io::stdout(), pixel_color, samples_per_pixel).unwrap();
        }
    }
    eprintln!("\nDone.");
}
