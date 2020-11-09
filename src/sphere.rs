use crate::vec3::*;
use crate::ray::*;
use crate::hittable::*;
use crate::hittable::Hittable;
use crate::material::Material;
// use std::rc::Rc;
use std::sync::Arc;



pub struct Sphere  {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Arc<Material>,
}

type IsFrontFace = bool;
type Normal = Vec3;

pub fn set_face_normal(ray:&Ray, outward_normal: Vec3) -> (IsFrontFace, Normal) {
    let front_face = ray.dir.dot(outward_normal) < 0.0;
    if front_face {
        return ( front_face, outward_normal);
    } else {
        return ( front_face, -outward_normal);
    }
}


impl Hittable for Sphere {

    fn hit(&self,ray:&Ray, tmin:f64, tmax:f64 ) -> Option<HitRecord>{
        let oc:Vec3 = ray.orig - self.center;
        let a = ray.dir.len_sqred();
        let half_b = oc.dot(ray.dir);
        let c = oc.len_sqred() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let mut temp = (-half_b - root)/a;
            if temp<tmax && temp>tmin{
                // hitrecord
                let temp_point = ray.clone().at(temp); 
                let outward_normal: Vec3 = (temp_point - self.center) / self.radius;// unit norm
                let (ff,norm)= set_face_normal(&ray,outward_normal);

                let hr = HitRecord{ 
                    p: ray.clone().at(temp), 
                    normal: norm, 
                    t: temp,
                    mat_ptr: self.mat_ptr.clone(),
                    front_face:ff
                };
                return Some(hr);
            }

            let mut temp = (-half_b + root)/a;
            if temp<tmax && temp>tmin {
                let temp_point = ray.clone().at(temp); 
                let outward_normal: Vec3 = (temp_point - self.center) / self.radius;// unit norm
                let (ff,norm)= set_face_normal(&ray,outward_normal);

                let hr = HitRecord{ 
                    p: ray.clone().at(temp), 
                    normal: norm, 
                    mat_ptr: self.mat_ptr.clone(),
                    t: temp, 
                    front_face:ff
                };
                return Some(hr);
            } ;
        }

        return None;
    }

}


