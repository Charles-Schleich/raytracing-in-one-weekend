use crate::vec3::*;

#[derive(Debug,Clone)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
 
    pub fn new(o: Point3, d: Vec3) -> Ray {

        Ray {
                dir: d,
                orig: o,
        }
    }

    pub fn at(self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}
