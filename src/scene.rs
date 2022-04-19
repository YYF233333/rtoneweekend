use crate::hittable_list::HittableList;
use crate::material::*;
use crate::rtweekend::random_float;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3};
use rand::random;
use std::sync::Arc;

pub fn test_scene() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Arc::new(TestMaterial::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.0,
        ground_material,
    )));

    let material = Arc::new(TestMaterial::new(Color::new(0.7, 0.6, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.0,
        material,
    )));
    let material = Arc::new(TestMaterial::new(Color::new(0.7, 0.6, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.0,
        material,
    )));
    let material = Arc::new(TestMaterial::new(Color::new(0.7, 0.6, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.0,
        material,
    )));

    world
}

pub fn random_scene() -> HittableList {
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
