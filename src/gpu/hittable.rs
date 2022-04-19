use std::sync::Arc;

use crate::material::Material;
use crate::ray::*;
use crate::vec3::*;

//class material;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
