
// Interal
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

//  External
use num_cpus;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::process::Command;

use rand::prelude::*;
use rayon::prelude::*;
// extern crate image;
use image::*;


// Image + Camera Stuff
const ASPECT_RATIO:f64 = 16.0 / 9.0;
const IMG_WIDTH:u32 = 1920;
// const IMG_WIDTH:i32 = 400;
const IMG_HEIGHT:u32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as u32 ;

// Anti-Aliasing + Recurse Bounce 
const SAMPLES_PER_PIXEL: i32 = 10;
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


fn process_line (row:f64, cam:Arc<Camera>, world: Arc<HittableList>) -> Vec<Colour> {
    // eprintln!("Runing Row {}",row );

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

                if choose_mat < 0.4 {
                    // diffuse
                    let albedo = Colour::random() * Colour::random();
                    material = Arc::new(Lambertian{ albedo:albedo });
                    world.add(Arc::new(Sphere{ center: center, radius: 0.2, mat_ptr:material}));
                      
                } else if choose_mat < 0.7 {
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
        }
    }

    let material1   = Arc::new(Dielectric{ ir: 1.5 });
    world.add(Arc::new(Sphere{ center: Point3{x:0.0,y:1.0,z:0.0}, radius: 1.0, mat_ptr:material1}));

    let material2 = Arc::new(Lambertian{ albedo:Colour{x:0.4,y:0.2,z:0.1}  });
    world.add(Arc::new(Sphere{ center: Point3{x:-4.0,y:1.0,z:0.0}, radius: 1.0, mat_ptr:material2}));

    let material3 = Arc::new(Metal{ albedo:Colour{x:0.7,y:0.6,z:0.5}, fuzz:0.0  });
    world.add(Arc::new(Sphere{ center: Point3{x:4.0,y:1.0,z:0.0}, radius: 1.0, mat_ptr:material3}));

    Arc::new(world)
}



fn render(cam:Arc<Camera>, world_arc: Arc<HittableList>, frame:String) {

    // eprintln!("{}",cam.lower_left_corner);
    println!("Rendering {}",frame);

    // Size 
    // eprintln!("size {} {}",IMG_HEIGHT, num_cpus::get());
    // print!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT);
    let rows = 0..IMG_HEIGHT;

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


    let mut rgbvec = Vec::new();

    for pixel_colour in output {
        let (r,g,b)= pixel_colour.write_colour(SAMPLES_PER_PIXEL);
        rgbvec.push((r,g,b))
    }

    let chunk_rows = rgbvec.chunks(IMG_WIDTH as usize);
    let mut imgbuf = image::ImageBuffer::new(IMG_WIDTH, IMG_HEIGHT);

    for (y, data) in chunk_rows.into_iter().enumerate() {
        for (x, (r,g,b)) in data.into_iter().enumerate() {
            // println!("{:?} {:?} {:?}",y,x , (r,g,b));
            imgbuf.put_pixel(x as u32, y as u32, image::Rgb([*r, *g, *b]));
        }
    }

    imgbuf.save(frame).unwrap();

}

fn main() {
    eprintln!("Starting Ray Tracing: W{}xH{}",IMG_WIDTH,IMG_HEIGHT);

    let world_arc = random_scene();


    // render 
    for n in 1..601{
        // Camera
        let t = (n as f64)/20.0;
        let x_coord = 13.0 - t;
        let y_coord = 2.0 + 0.1*t;
        let z_coord = 3.0 - t;

        let lookfrom = Point3 { x:x_coord, y:y_coord, z: z_coord};
        let lookat   = Point3 { x:0.0,  y:1.0, z: 0.0};
        let vup      = Point3 { x:0.0,  y:1.0, z: 0.0};
        let dist_to_focus =  10.0;
        let aperture  =  0.1;

        // point3(-2,2,1), point3(0,0,-1)
        let cam = Arc::new(Camera::new(lookfrom,lookat,vup, 20.0,ASPECT_RATIO,aperture,dist_to_focus));

        let mut frame = "frame".to_string();
        frame.push_str(&n.to_string());
        frame.push_str(".png");
        println!("{:?}",lookfrom);

        let start = Instant::now();

            render(cam, world_arc.clone(), frame);
        let duration = start.elapsed();
        println!("Frame Time: {:?}s ", duration.as_secs());
        println!("----------------");

    }

  
}
