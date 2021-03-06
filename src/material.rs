
use rand::prelude::*;
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
// fn reflect( v : Vec3, n : Vec3  ) -> Vec3 {
//     return  v - 2.0*v.dot(n)*n;
// }
pub struct Metal {
    pub albedo: Colour,
    pub fuzz  : f64,   
}

impl Material for Metal {
    fn scatter(&self, ray_in : &Ray, hit_record: &HitRecord) -> Option<(Ray,Colour)> {
        let reflected  = Vec3::reflect(Vec3::unit_vector(ray_in.dir),hit_record.normal);
        let ray =  Ray::new(hit_record.p, reflected+ self.fuzz*Vec3::random_in_unit_sphere());
        return Some((ray,self.albedo))
    }
}




//  Dielectric
pub struct Dielectric {
    pub ir:f64
}

fn reflectance(cosine:f64, ref_idx:f64) -> f64 {
    // Using Schlicks approximation for reflectance
    let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
    r0 = r0*r0;

    return r0 + (1.0-r0)*f64::powi(1.0-cosine,5);//((1-cosine),5);
}


impl Material for Dielectric {
    fn scatter(&self, ray_in : &Ray, hit_record: &HitRecord) -> Option<(Ray,Colour)> {
        let mut rng = rand::thread_rng();
        let attenuation = Vec3{ x:1.0, y:1.0, z:1.0};  

        // Is this coming into or out of the di-electric ?
        // Dielectric of Air is 1.0
        let refraction_ratio = match hit_record.front_face{
            true  => { 1.0/self.ir }
            false => { self.ir     }
        };

        let unit_direction = unit_vector(ray_in.dir);

        let cos_theta = -unit_direction.dot(hit_record.normal);
        // .min(1.0)
      // double cos_theta = fmin(dot(-unit_direction, rec.normal), 1.0);

        let sin_theta = f64::sqrt(1.0 - cos_theta* cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction;

        if cannot_refract || reflectance(cos_theta,refraction_ratio) > rng.gen::<f64>()   {
            direction = Vec3::reflect(unit_direction, hit_record.normal)
        } else {
            direction = Vec3::refract(unit_direction,hit_record.normal,refraction_ratio);
        }

        let scattered_ray = Ray::new(hit_record.p, direction);
        // I.e. no attention todo: add code to make di-electric a coloured sphere i.e. rose tinted

        return Some((scattered_ray,attenuation));

    }
}


