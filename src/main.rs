use std::io::Write;

use rand::random;
use vec3::{Color, Point3};

use crate::{color::write_color, vec3::Vec3};
use camera::*;
use ray_color::*;
use scene::*;

mod camera;
mod color;
mod gpu;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod ray_color;
mod rtweekend;
mod scene;
mod sphere;
mod vec3;

#[allow(non_upper_case_globals)]
fn main() {
    // Image
    static aspect_ratio: f32 = 3.0 / 2.0;
    static image_width: i32 = 600;
    static image_height: i32 = (image_width as f32 / aspect_ratio) as i32;
    static samples_per_pixel: i32 = 100;
    static max_depth: i32 = 50;

    // World

    let world = test_scene();

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
            let rays: Vec<_> = (0..samples_per_pixel)
                .into_iter()
                .map(|_| {
                    let u = (i as f32 + random::<f32>()) / (image_width - 1) as f32;
                    let v = (j as f32 + random::<f32>()) / (image_height - 1) as f32;
                    cam.get_ray(u, v)
                })
                .collect();
            let pixel_color = ray_color(rays, &world, max_depth)
                .into_iter()
                .reduce(|acc, x| acc + x)
                .unwrap();
            write_color(&mut std::io::stdout(), pixel_color, samples_per_pixel).unwrap();
        }
    }
    eprintln!("\nDone.");
}
