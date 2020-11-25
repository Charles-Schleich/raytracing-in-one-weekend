
use crate::vec3::*;
use crate::ray::*;
use std::f64::consts::PI;

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Point3, 
    pub lower_left_corner: Point3, 
    pub horizontal: Vec3, 
    pub vertical: Point3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,

}




impl Camera {

    pub fn new( lookfrom:Point3
              , lookat:Point3
              , vup:Vec3
              , vfov:f64
              , aspect_ratio:f64
              , aperture:f64
              , focus_dist:f64
              ) -> Camera {

        let theta = vfov.to_radians();
        let h = f64::tan(theta/2.0);
        let viewport_height = 2.0 * h; 
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(vup.cross(w));
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width  * u; 
        let vertical   = focus_dist * viewport_height * v;

        let lower_left_corner = origin
            - horizontal / 2.0
            - vertical / 2.0
            - focus_dist * w;

        Camera {
            origin: origin, 
            lower_left_corner: lower_left_corner, 
            horizontal: horizontal, 
            vertical: vertical,
            u: u,
            v: v,
            w: w,
            lens_radius: aperture/2.0,
        }
    }


    pub fn getray(self, s:f64, t:f64) -> Ray {
        let raydir:Vec3 = self.lens_radius * Vec3::random_in_unit_disk();
        let offset:Vec3 = self.u * raydir.x + self.v * raydir.y;

        // let raydir:Vec3 =     self.lower_left_corner 
        //                 + s * self.horizontal 
        //                 + t * self.vertical - self.origin;
        

        Ray{
            orig: self.origin + offset,
            dir:  self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset 
            
        }
    }

}
