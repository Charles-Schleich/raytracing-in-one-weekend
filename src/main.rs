use std::fs;
pub mod vec3;
use vec3::*;

fn main() {
    let a = Vec3::new();
    let b = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let c = b.len();

    // eprint!("New Vec {}\n ",c);
    // return ;

    let img_width = 256;
    let img_height = 256;

    // Render
    print!("P3\n{} {}\n255\n", img_width, img_height);
    for row in (0..img_height).rev() {
        eprintln!("Lines Remaining {}", row);
        for col in 0..img_width {
            let pixel_colour: Vec3 = Colour {
                x: col as f64 / (img_width) as f64,
                y: row as f64 / (img_height) as f64,
                z: 0.25,
            };

            pixel_colour.write_colour()
        }
    }
}
