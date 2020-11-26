
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
// const IMG_WIDTH:i32 = 400;
const IMG_HEIGHT:i32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as i32 ;

// Anti-Aliasing + Recurse Bounce 
const SAMPLES_PER_PIXEL: i32 = 80;
// const SAMPLES_PER_PIXEL: i32 = 80;
const MAX_RAY_BOUNCE:u8 = 10;


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
    eprintln!("Runing Row {}",row );

    let mut values:Vec<Colour> = Vec::new();
    let mut rng = rand::thread_rng();

    for col in 0..IMG_WIDTH { 
        let mut pixel_colour: Colour = Colour::new();

        for _ in 1..SAMPLES_PER_PIXEL {
            let u = (col as f64 + rng.gen::<f64>() ) / (IMG_WIDTH) as f64;
            let v = (row as f64 + rng.gen::<f64>() ) / (IMG_HEIGHT) as f64;

            let ray= cam.getray(u, v);
            pixel_colour = pixel_colour+ray_colour(ray, &world,MAX_RAY_BOUNCE); 
        }
        values.push(pixel_colour);
    }
    values
}




fn random_scene() -> Arc<HittableList> {

    let mut world: HittableList = HittableList::new();
    let mat_ground = Arc::new(Lambertian{ albedo:Colour{x:0.5,y:0.5,z:0.5} });
    world.add(Arc::new(Sphere{center: Point3{x: 0.0,y:-1000.0,z:0.0},radius: 1000.0, mat_ptr:mat_ground}));

    let mut rng = rand::thread_rng();

    for x in -11..11{
        for z in -11..11{
            let choose_mat = rng.gen::<f64>();
            let center = Point3{ x : x as f64 + 0.9*rng.gen::<f64>() 
                                    , y : 0.2
                                    , z : z as f64 + 0.9*rng.gen::<f64>()
                                    };


            if (center - Point3{x:4.0, y:0.2, z:0.0}).len() > 0.9 {

                let material:Arc<Material>;

                if choose_mat < 0.6 {
                    // diffuse
                    let albedo = Colour::random() * Colour::random();
                    material = Arc::new(Lambertian{ albedo:albedo });
                    world.add(Arc::new(Sphere{ center: center, radius: 0.2, mat_ptr:material}));
                      
                } else if choose_mat < 0.8 {
                    // metal
                    let albedo = Colour::random() * Colour::random();
                    let fuzz = rng.gen_range(0.0, 0.2);
                    material =  Arc::new(Metal{ albedo:albedo, fuzz: fuzz });
                    world.add(Arc::new(Sphere{ center: center, radius: 0.2, mat_ptr:material}));
                } else {
                    // glass
                    let material   = Arc::new(Dielectric{ ir: 1.5 });
                    world.add(Arc::new(Sphere{ center: center, radius: 0.2, mat_ptr:material}));
                }
            }
            // auto choose_mat = random_double();
            // point3 center(a + 0.9*random_double(), 0.2, b + 0.9*random_double());

        }
    }
    // world.add(Arc::new((point3(0, 1, 0), 1.0, material1));
    let material1   = Arc::new(Dielectric{ ir: 1.5 });
    world.add(Arc::new(Sphere{ center: Point3{x:0.0,y:1.0,z:0.0}, radius: 1.0, mat_ptr:material1}));

    let material2 = Arc::new(Lambertian{ albedo:Colour{x:0.4,y:0.2,z:0.1}  });
    world.add(Arc::new(Sphere{ center: Point3{x:-4.0,y:1.0,z:0.0}, radius: 1.0, mat_ptr:material2}));


    let material3 = Arc::new(Metal{ albedo:Colour{x:0.7,y:0.6,z:0.5}, fuzz:0.0  });
    world.add(Arc::new(Sphere{ center: Point3{x:4.0,y:1.0,z:0.0}, radius: 1.0, mat_ptr:material3}));

    // auto material3 = make_shared<metal>(color(0.7, 0.6, 0.5), 0.0);
    // world.add(make_shared<sphere>(point3(4, 1, 0), 1.0, material3));

    // return world;
    Arc::new(world)
}




fn main() {
    eprintln!("Starting Ray Tracing: W{}xH{}",IMG_WIDTH,IMG_HEIGHT);

    // Materials
    // let mat_ground = Arc::new(Lambertian{ albedo:Colour{x:0.8,y:0.8,z:0.0} });
    // let mat_center: Arc<Lambertian> = Arc::new(Lambertian{ albedo:Colour{x:0.1,y:0.2,z:0.5} });
    // let mat_left   = Arc::new(Dielectric{ ir: 1.5 });
    // let mat_right =  Arc::new(Metal{ albedo:Colour{x:0.8,y:0.6,z:0.2}, fuzz: 0.0 });

    // // World
    // let mut world: HittableList = HittableList::new();
    // world.add(Arc::new(Sphere{center: Point3{x: 0.0,y:-100.5,z:-1.0},radius: 100.0, mat_ptr:mat_ground}));
    // world.add(Arc::new(Sphere{center: Point3{x: 0.0,y:0.0,z:-1.0}   ,radius: 0.5,   mat_ptr:mat_center}));
    // world.add(Arc::new(Sphere{center: Point3{x:-1.0,y:0.0,z:-1.0}   ,radius: 0.5,   mat_ptr:mat_left.clone()}));
    // world.add(Arc::new(Sphere{center: Point3{x:-1.0,y:0.0,z:-1.0}   ,radius: -0.45, mat_ptr:mat_left}));
    // world.add(Arc::new(Sphere{center: Point3{x: 1.0,y:0.0,z:-1.0}   ,radius: 0.5,   mat_ptr:mat_right}));

    // let world_arc = Arc::new(world);
    let world_arc = random_scene();

    // Camera
    let lookfrom = Point3 { x:13.0, y:2.0, z: 2.0};
    let lookat   = Point3 { x:0.0,  y:0.0, z: 0.0};
    let vup      = Point3 { x:0.0,  y:1.0, z: 0.0};
    let dist_to_focus =  10.0;
    let aperture  =  0.1;

    // point3(-2,2,1), point3(0,0,-1)
    let cam = Arc::new(Camera::new(lookfrom,lookat,vup, 20.0,ASPECT_RATIO,aperture,dist_to_focus));



    eprintln!("{}",cam.lower_left_corner);

    // Size 
    eprintln!("size {} {}",IMG_HEIGHT, num_cpus::get());
    print!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT);
    let rows = (0..IMG_HEIGHT);

    // Rayon splitting up the work to a couple cores. 
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
