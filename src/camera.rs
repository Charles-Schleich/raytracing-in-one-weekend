
use crate::vec3::*;
use crate::ray::*;

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Point3, 
    pub lower_left_corner: Point3, 
    pub horizontal: Vec3, 
    pub vertical: Point3, 
}



impl Camera {

    pub fn new(aspect_ratio:f64) -> Camera{
        let viewport_height = 2.0; 
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new();
        let horizontal = Vec3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let vertical = Vec3 {
            x: 0.0,
            y: viewport_height,
            z: 0.0,
        };
        let lower_left_corner = origin
            - horizontal / 2.0
            - vertical / 2.0
            - Vec3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            };

        Camera {
            origin: Vec3{x:0.0,y:0.0,z:0.0}, 
            lower_left_corner: lower_left_corner, 
            horizontal: horizontal, 
            vertical: vertical, 
        }
    }


    pub fn getray(self, u:f64,v:f64) -> Ray {
        let raydir = self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin;
        // Ray::new(self.origin.clone(),raydir)
        Ray{
            orig:self.origin,
            dir: raydir
        }
    }





}
