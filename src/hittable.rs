use crate::vec3::*;
use crate::ray::*;

pub struct Hittable{
    pub p: Point3,
    pub normal: vec3,
    pub t: f64,
    pub front_face:bool,
}

impl Hittable {

    fn set_face_normal(self, ray:Ray, outward_normal: vec3){
        self.front_face = r.dir.dot(outward_normal) < 0;
        if self.front_face {
            normal = outward_normal;
        } else {
            normal = -outward_normal;
        }

    }

}

