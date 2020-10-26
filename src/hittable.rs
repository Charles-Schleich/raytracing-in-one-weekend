use crate::vec3::*;
use crate::ray::*;
use crate::material::*;
use std::rc::Rc;
// std::sync::Arc

// #[derive(Debug,Copy,Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Rc<Material>,
    pub t: f64,
    pub front_face:bool,
}



impl HitRecord {

    pub fn set_face_normal(mut self, ray:&Ray, outward_normal: Vec3){
        self.front_face = ray.dir.dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin:f64, tmax:f64 ) -> Option<HitRecord>;
}

//
pub struct HittableList{
    objects: Vec<Box<Hittable>>
}

impl HittableList{

    pub fn new() -> HittableList {
        HittableList{
            objects:Vec::new()
        }
    }

    pub fn clear(mut self){
        self.objects = Vec::new();
    }

    pub fn add(&mut self, sharedptr:Box<Hittable> ){
        self.objects.push(sharedptr);
    }
}

impl Hittable for HittableList{

    // const ray& r, double t_min, double t_max, hit_record& rec)
    fn hit(&self,ray:&Ray, tmin:f64, tmax:f64, ) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = tmax;
        
        for object in self.objects.iter() {
            if let Some(hit) = object.hit(&ray, tmin, closest_so_far) {
                
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }
        return hit_anything;
    }

}
