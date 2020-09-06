use crate::vec3::*;
use crate::ray::*;
use crate::hittable::*;


struct Sphere{
    cen: Point3,
    r: f64
}


impl Sphere {

    fn hit (self, r:Ray, tmin:f64, tmax:f64, mut rec: hit_record ) -> bool {
        let oc:Vec3 = ray.orig - self.center;
        let a = ray.dir.len_sqred();
        let half_b = oc.dot(ray.dir);
        let c = oc.len_sqred() - rad*rad;
        let discriminant = half_b*half_b - a*c;

        if discriminant > 0.0{
            let root = discriminant.sqrt();

            let mut temp = (-half_b - root)/a;
            if temp<tmax && temp>t_min{
                rec.t = temp;
                rec.p = r.at(tec.t);
                rec.normal= (rec.p-self.center)/radius ; // unit norm
                let outward_normal = (rec.p- center) / radius;
                rec.set_face_normal(r,outward_normal);
                return true;
            }

            let mut temp = (-half_b + root)/a;
            if temp<tmax && temp>t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                rec.normal = (rec.p - center) / radius;
                let outward_normal = (rec.p- center) / radius;
                rec.set_face_normal(r,outward_normal);
                return true;
            } ;
        }
        return false;
    }

}