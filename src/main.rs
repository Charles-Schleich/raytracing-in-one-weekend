pub mod vec3;
pub mod ray;
pub mod sphere;
pub mod hittable;
pub mod camera;

use vec3::*;
use ray::*;
use hittable::*;
use sphere::*;

use camera::*;
use rand::prelude::*;


// Image + Camera Stuff
const ASPECT_RATIO:f64 = 16.0 / 9.0;
const IMG_WIDTH:i32 = 400;
const IMG_HEIGHT:i32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as i32 ;

// Anti-Aliasing + Recurse Bounce 
const SAMPLES_PER_PIXEL: i32 = 8;
const MAX_RAY_BOUNCE:u8 = 3;


fn ray_colour(r: Ray, world: &HittableList, depth:u8) -> Colour {
    // exceeded the hit depth, no more adding light
    if depth <= 0 {
        return Colour {x: 0.0,y: 0.0,z: 0.0};
    }

    let hitrecord = world.hit(&r,  0.001 , f64::MAX);

    if  hitrecord.is_some() {
        let hr = hitrecord.unwrap();
        let target: Point3 = hr.p  + hr.normal + Vec3::random_in_unit_sphere();
        // bounce ray off and calculate next hit
        return 0.5 * ray_colour(Ray::new(hr.p, target-hr.p), world, depth-1)
        // return 0.5 * (hr.normal + Colour{x:1.0,y:1.0,z:1.0})
    }

    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    // linear fade between white and blue (blue at t, white at 0->t)
    let sky = (1.0 - t) * Colour {x: 1.0,y: 1.0,z: 1.0,} + t * Colour{x: 0.5,y: 0.7,z: 1.0};
    return sky;
}


fn main() {

    // World 
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere{center: Point3{x:0.0,y:0.0,z:-1.0}   ,radius: 0.5}));
    world.add(Box::new(Sphere{center: Point3{x:0.0,y:-100.5,z:-1.0},radius: 100.0}));

    // Camera
    let cam = Camera::new(ASPECT_RATIO);
    let mut rng = rand::thread_rng();

    eprintln!("{}",cam.lower_left_corner);
    // Render
    print!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT);
    //
    for row in (0..IMG_HEIGHT).rev() {
        // eprintln!("Lines Remaining {}", row);
        for col in 0..IMG_WIDTH {

            let mut pixel_colour: Colour = Colour::new();

            for _ in 1..SAMPLES_PER_PIXEL{
                let u = (col as f64 + rng.gen::<f64>() ) / (IMG_WIDTH) as f64;
                let v = (row as f64 + rng.gen::<f64>() ) / (IMG_HEIGHT) as f64;

                let ray= cam.getray(u, v);
                pixel_colour = pixel_colour+ray_colour(ray,&world,MAX_RAY_BOUNCE); 
            }

            pixel_colour.write_colour(SAMPLES_PER_PIXEL);
        }
    }
}
