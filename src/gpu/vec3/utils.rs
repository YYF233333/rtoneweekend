use super::vec3::*;
use crate::rtweekend::*;

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_in_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit(random_in_unit_sphere())
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(in_unit_sphere, *normal) > 0.0
    // In the same hemisphere as the normal
    {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random_float(-1.0, 1.0), random_float(-1.0, 1.0), 0.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}
