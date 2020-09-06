pub mod vec3;
pub mod ray;
pub mod sphere;
pub mod hittable;
pub mod rtweekend;

use vec3::*;
use ray::*;
use hittable::*;
use sphere::*;

// Image stuff
const ASPECT_RATIO:f64 = 16.0 / 9.0;
const IMG_WIDTH:i32 = 1600;
const IMG_HEIGHT:i32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as i32 ;

fn ray_colour(r: Ray, world: &HittableList) -> Colour {
    let hitrecord = world.hit(&r, 0.0 , f64::MAX);
    if  hitrecord.is_some() {
        let hr = hitrecord.unwrap();
        return 0.5 * (hr.normal + Colour{x:1.0,y:1.0,z:1.0})
    }


    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    let sky = (1.0 - t) * Colour {x: 1.0,y: 1.0,z: 1.0,} + t*Colour{x: 0.5,y: 0.7,z: 1.0};
    return sky;
}

fn main() {

    // World 
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere{center: Point3{x:0.0,y:0.0,z:-1.0},radius: 0.5}));
    world.add(Box::new(Sphere{center: Point3{x:0.0,y:-100.5,z:-1.0},radius: 100.0}));

    // Camera
    let viewport_height = 2.0; 
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new();
    let horizontal = Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };

    eprintln!("{}",lower_left_corner);

    // Render
    print!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT);
    //
    for row in (0..IMG_HEIGHT).rev() {
        // eprintln!("Lines Remaining {}", row);
        for col in 0..IMG_WIDTH {
            let u =  col as f64 / (IMG_WIDTH) as f64;
            let v = row as f64 / (IMG_HEIGHT) as f64;
            let ray: Ray = Ray {
                orig: origin,
                dir: lower_left_corner + u*horizontal + v*vertical - origin,
            };

            let pixel_colour: Vec3 = ray_colour(ray,&world);
            pixel_colour.write_colour()
        }
    }
}
