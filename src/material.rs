
use crate::hittable::*;
use crate::vec3::*;
use crate::ray::*;


pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord ) -> Option<(Ray,Colour)> ;
}


// Diffuse
pub struct Lambertian {
    pub albedo: Colour
}


impl Material for Lambertian {
    fn scatter(&self, _ : &Ray, hit_record: &HitRecord) -> Option<(Ray,Colour)> {
        let scatter_direction = hit_record.normal + Vec3::random_in_unit_vector();
        let ray =  Ray::new(hit_record.p, scatter_direction);
        return Some((ray,self.albedo))
    }
}



fn reflect( v : Vec3, n : Vec3  ) -> Vec3 {
    return  v - 2.0*v.dot(n)*n;
}
pub struct Metal {
    pub albedo: Colour
}

impl Material for Metal {
    fn scatter(&self, ray_in : &Ray, hit_record: &HitRecord) -> Option<(Ray,Colour)> {
        let reflected  = reflect(Vec3::unit_vector(ray_in.dir),hit_record.normal);
        let ray =  Ray::new(hit_record.p, reflected);
        return Some((ray,self.albedo))
    }
}
