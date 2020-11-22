
use crate::vec3::*;
use crate::ray::*;
use std::f64::consts::PI;

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Point3, 
    pub lower_left_corner: Point3, 
    pub horizontal: Vec3, 
    pub vertical: Point3, 
}




impl Camera {

    pub fn new(lookfrom:Point3, lookat:Point3, vup:Vec3, vfov:f64, aspect_ratio:f64) -> Camera {

        let theta = vfov.to_radians();
        let h = f64::tan(theta/2.0);
        let viewport_height = 2.0 * h; 
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(vup.cross(w));
        let v = w.cross(u);
        let origin = lookfrom;
        let horizontal = viewport_width  * u; 
        let vertical   = viewport_height * v;

        let lower_left_corner = origin
            - horizontal / 2.0
            - vertical / 2.0
            - w;

        Camera {
            origin: origin, 
            lower_left_corner: lower_left_corner, 
            horizontal: horizontal, 
            vertical: vertical, 
        }
    }


    pub fn getray(self, u:f64, v:f64) -> Ray {
        let raydir:Vec3 = self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin;
        // Ray::new(self.origin.clone(),raydir)
        Ray{
            orig:self.origin,
            dir: raydir
        }
    }

}
