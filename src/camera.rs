use crate::{ray::Ray, vec3::*};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let mut cam = Self {
            origin: look_from,
            w: unit(look_from - look_at),
            ..Default::default()
        };
        cam.u = unit(cross(vup, cam.w));
        cam.v = cross(cam.w, cam.u);
        cam.horizontal = focus_dist * viewport_width * cam.u;
        cam.vertical = focus_dist * viewport_height * cam.v;
        cam.lower_left_corner =
            cam.origin - cam.horizontal / 2. - cam.vertical / 2. - focus_dist * cam.w;

        cam.lens_radius = aperture / 2.;
        cam
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
