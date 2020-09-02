
use std::fs;

fn main() {

    let img_width  = 256;
    let img_height  = 256;

    // Render
    print!("P3\n{} {}\n255\n",img_width,img_height);
    for row in (0..img_height).rev(){
        eprintln!("Lines Remaining {}", row);
        for col in 0..img_width{
            let r:f64 = col as f64/(img_width) as f64 ;
            let g:f64 = row as f64/(img_height) as f64;
            let b:f64 = 0.25;

            let i_r = (256.0*r).round() as u8; 
            let i_b = (256.0*b).round() as u8; 
            let i_g = (256.0*g).round() as u8; 

            print!("{} {} {}\n",i_r,i_g,i_b);

        }
    }

}
