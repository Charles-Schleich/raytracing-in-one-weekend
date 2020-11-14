
use std::rc::Rc;
use num_cpus;
use std::thread;
use std::sync::Arc;

pub mod vec3;
pub mod ray;
pub mod sphere;
pub mod hittable;
pub mod camera;
pub mod material;

use vec3::*;
use ray::*;
use hittable::*;
use sphere::*;
use material::*;


use camera::*;
use rand::prelude::*;
use rayon::prelude::*;


// Image + Camera Stuff
const ASPECT_RATIO:f64 = 16.0 / 9.0;
const IMG_WIDTH:i32 = 400;
const IMG_HEIGHT:i32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as i32 ;

// Anti-Aliasing + Recurse Bounce 
const SAMPLES_PER_PIXEL: i32 = 80;
const MAX_RAY_BOUNCE:u8 = 5;


fn ray_colour(r: Ray, world: &HittableList, depth:u8) -> Colour {
    // exceeded the hit depth, no more adding light
    if depth <= 0 {
        return Colour {x: 0.0,y: 0.0,z: 0.0};
    }

    let hitrecord = world.hit(&r,  0.001 , f64::MAX);

    if  hitrecord.is_some() {
        let hr = hitrecord.unwrap();

        let opt_scatter_attenuation = hr.mat_ptr.scatter(&r, &hr);

        match opt_scatter_attenuation {
            Some((ray,attenuation)) =>{ return attenuation*ray_colour(ray, world, depth-1)}
            None =>{ return Colour{x:0.0,y:0.0,z:0.0}}
        }

    }

    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    // linear fade between white and blue (blue at t, white at 0->t)
    let sky = (1.0 - t) * Colour {x: 1.0,y: 1.0,z: 1.0,} + t * Colour{x: 0.5,y: 0.7,z: 1.0};
    return sky;
}

#[derive(Debug)]
struct ThreadBounds {
    thread: i32,
    start: i32,
    end: i32,
}


// fn process_image_chunk (tb:ThreadBounds, cam:Arc<Camera>, world: Arc<HittableList>) -> Vec<Colour>{
fn process_line (row:f64, cam:Arc<Camera>, world: Arc<HittableList>) -> Vec<Colour> {

    let mut values:Vec<Colour> = Vec::new();
    let mut rng = rand::thread_rng();

    for col in 0..IMG_WIDTH { 
        let mut pixel_colour: Colour = Colour::new();

        for _ in 1..SAMPLES_PER_PIXEL {
            let u = (col as f64 + rng.gen::<f64>() ) / (IMG_WIDTH) as f64;
            let v = (row as f64 + rng.gen::<f64>() ) / (IMG_HEIGHT) as f64;

            let ray= cam.getray(u, v);
            pixel_colour = pixel_colour+ray_colour(ray,&world,MAX_RAY_BOUNCE); 
        }
        values.push(pixel_colour);
    }
    values
}



fn main() {
    eprintln!("Starting Ray Tracing: W{}xH{}",IMG_WIDTH,IMG_HEIGHT);

    // Materials
    let mat_ground = Arc::new(Lambertian{ albedo:Colour{x:0.8,y:0.8,z:0.0} });
    // let mat_center: Arc<Lambertian> = Arc::new(Lambertian{ albedo:Colour{x:0.7,y:0.3,z:0.3} });
    let mat_center = Arc::new(Dielectric{ ir: 1.5 });
    let mat_left   = Arc::new(Dielectric{ ir: 1.5 });
    let mat_right =  Arc::new(Metal{ albedo:Colour{x:0.1,y:0.1,z:0.7}, fuzz: 1.0 });
    // let mat_right =  Arc::new(Metal{ albedo:Colour{x:0.8,y:0.6,z:0.2}, fuzz: 1.0 });


    // World
    let mut world: HittableList = HittableList::new();
    world.add(Arc::new(Sphere{center: Point3{x: 0.0,y:-100.5,z:-1.0},radius: 100.0, mat_ptr:mat_ground}));
    world.add(Arc::new(Sphere{center: Point3{x: 0.0,y:0.0,z:-1.0}   ,radius: 0.5,   mat_ptr:mat_center}));
    world.add(Arc::new(Sphere{center: Point3{x:-1.0,y:0.0,z:-1.0}   ,radius: 0.5,   mat_ptr:mat_left}));
    world.add(Arc::new(Sphere{center: Point3{x: 1.0,y:0.0,z:-1.0}   ,radius: 0.5,   mat_ptr:mat_right}));

    let world_arc = Arc::new(world);
    // Camera
    let cam = Arc::new(Camera::new(ASPECT_RATIO));

    eprintln!("{}",cam.lower_left_corner);

    // Render

    // Split up work 
    

    // size 
    eprintln!("size {} {}",IMG_HEIGHT, num_cpus::get());
    print!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT);
    let rows = (0..IMG_HEIGHT);


    let output: Vec<Colour> = rows
        .into_par_iter()
        .rev()
        .flat_map(|x|{
            let cam_cl= cam.clone();
            let world_arc_cl= world_arc.clone();
            process_line(x as f64,cam_cl,world_arc_cl)
        })
        .collect();

    for pixel_colour in output{
        pixel_colour.write_colour(SAMPLES_PER_PIXEL);
    }

    // process_line

    //
    // for row in (0..IMG_HEIGHT).rev() {
    //     // eprintln!("Lines Remaining {}", row);
    //     for col in 0..IMG_WIDTH { 

    //         let mut pixel_colour: Colour = Colour::new();

    //         for _ in 1..SAMPLES_PER_PIXEL {
    //             let u = (col as f64 + rng.gen::<f64>() ) / (IMG_WIDTH) as f64;
    //             let v = (row as f64 + rng.gen::<f64>() ) / (IMG_HEIGHT) as f64;

    //             let ray= cam.getray(u, v);
    //             pixel_colour = pixel_colour+ray_colour(ray,&world,MAX_RAY_BOUNCE); 
    //         }

            // pixel_colour.write_colour(SAMPLES_PER_PIXEL);
    //     }
    // }
}
