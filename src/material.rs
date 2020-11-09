
use crate::hittable::*;
use crate::vec3::*;
use crate::ray::*;


pub trait Material : Send + Sync  {
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


//  Metalic
fn reflect( v : Vec3, n : Vec3  ) -> Vec3 {
    return  v - 2.0*v.dot(n)*n;
}
pub struct Metal {
    pub albedo: Colour,
    pub fuzz  : f64,   
}

impl Material for Metal {
    fn scatter(&self, ray_in : &Ray, hit_record: &HitRecord) -> Option<(Ray,Colour)> {
        let reflected  = reflect(Vec3::unit_vector(ray_in.dir),hit_record.normal);
        let ray =  Ray::new(hit_record.p, reflected+ self.fuzz*Vec3::random_in_unit_sphere());
        return Some((ray,self.albedo))
    }
}


//  Dielectric
pub struct Dielectric {
    pub ir:f64
}

impl Material for Dielectric {
    fn scatter(&self, ray_in : &Ray, hit_record: &HitRecord) -> Option<(Ray,Colour)> {
        // eprintln!("Hit Dielectric");
        // Is this coming into or out of the di-electric ?
        // Dielectric of Air is 1.0
        let refraction_ratio = match hit_record.front_face{
            true  => { 1.0/self.ir }
            false => { self.ir     }
        };

        let unit_direction = unit_vector(ray_in.dir);

        let refracted = Vec3::refract(unit_direction,hit_record.normal,refraction_ratio);

        let refracted_ray= Ray::new(hit_record.p , refracted);

        // I.e. no attention todo: add code to make di-electric a coloured sphere i.e. rose tinted
        let attenuation = Vec3{ x:1.0, y:1.0, z:1.0};  

        return Some((refracted_ray,attenuation));

    }
}


