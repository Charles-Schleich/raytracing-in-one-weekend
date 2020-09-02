use std::fs;
pub mod vec3;
use vec3::*;

pub mod ray;
use ray::*;

fn ray_colour(r: Ray) -> Colour {
    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    let colour = (1.0 - t) * Colour {x: 1.0,y: 1.0,z: 1.0,} + t*Colour{x: 0.5,y: 0.7,z: 1.0};
    // eprintln!("{} {}",t,colour);
    return colour;
}

fn main() {
    // Image stuff
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;
    let img_height = (img_width as f64 / aspect_ratio).round() as i32 ;
    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
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
    // (0,0,0) - (1.8,0,0)      - (0,1,0)      - (0,0,1)  = (-1.8,-1,-1)

    // Render
    print!("P3\n{} {}\n255\n", img_width, img_height);
    //
    for row in (0..img_height).rev() {
        eprintln!("Lines Remaining {}", row);
        for col in 0..img_width {
            let u =  col as f64 / (img_width) as f64;
            let v = row as f64 / (img_height) as f64;
            let ray: Ray = Ray {
                orig: origin,
                dir: lower_left_corner + u*horizontal + v*vertical - origin,
            };

            let pixel_colour: Vec3 = ray_colour(ray);
            pixel_colour.write_colour()
        }
    }
}
