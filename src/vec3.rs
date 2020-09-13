use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};
use rand::prelude::*;


pub type Point3 = Vec3;
pub type Colour = Vec3;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn dot(mut self, other: Vec3) -> f64 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.len()
    }

    pub fn len(self) -> f64 {
        f64::sqrt(self.len_sqred())
    }

    pub fn len_sqred(self) -> f64 {
        return &self.x * &self.x + &self.y * &self.y + &self.z * &self.z;
    }

    // random vector
    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        return  Vec3 {
            x: rng.gen::<f64>(),
            y: rng.gen::<f64>(),
            z: rng.gen::<f64>(),
        }
    }


    pub fn random_range(min:f64,max:f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        return  Vec3 {
            x: rng.gen_range(min, max),
            y: rng.gen_range(min, max),
            z: rng.gen_range(min, max),
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Vec3::random_range(-1.0, 1.0);
            if v.len_sqred() >= 1.0 {continue;} 
            return v;
        }
    }


} // end of impl for Vec3





impl Colour{
  pub fn write_colour(self, samples_per_pixel:i32) {
        let mut r = self.x;
        let mut g = self.y;
        let mut b = self.z;

        // divide the colour by number of samples
        let scale = 1.0 / samples_per_pixel as f64;
        // the sqrt is to Gamma correct for gamma 2.0
        r = f64::sqrt(r*scale);
        g = f64::sqrt(g*scale);        
        b = f64::sqrt(b*scale);        


        let i_r = (255.999 * clamp(r,0.0,0.999)).round() as u16;
        let i_g = (255.999 * clamp(g,0.0,0.999)).round() as u16;
        let i_b = (255.999 * clamp(b,0.0,0.999)).round() as u16;
        print!("{} {} {}\n", i_r, i_g, i_b)
    }
}


fn clamp(x:f64, min:f64, max:f64) -> f64 {
    if x < min { return min; }
    if x > max { return max; }
    return x;
}


// 
impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -&self.x,
            y: -&self.y,
            z: -&self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: &self.x - other.x,
            y: &self.y - other.y,
            z: &self.z - other.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: &self.x + other.x,
            y: &self.y + other.y,
            z: &self.z + other.z,
        }
    }
}

// Convenience
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            x: &self.x * other,
            y: &self.y * other,
            z: &self.z * other,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: &self.x * other.x,
            y: &self.y * other.y,
            z: &self.z * other.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            x: &self.x / other,
            y: &self.y / other,
            z: &self.z / other,
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

pub fn unit_vector(mut v:Vec3) -> Vec3 {
    v/ v.len()
}






// TEST
#[test]
fn test_add_vecs() {
    let a = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let b = Vec3 {
        x: 4.0,
        y: 5.0,
        z: 6.0,
    };
    let c = Vec3 {
        x: 5.0,
        y: 7.0,
        z: 9.0,
    };
    assert_eq!(a + b, c);
}

// TEST
#[test]
fn test_mult_vecs() {
    let a = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let b = Vec3 {
        x: 4.0,
        y: 5.0,
        z: 6.0,
    };
    let c = Vec3 {
        x: 4.0,
        y: 10.0,
        z: 18.0,
    };
    assert_eq!(a * b, c);
}

#[test]
fn test_div_vec_scalar() {
    let a = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let b = 5.0;
    let c = Vec3 {
        x: 1.0 / 5.0,
        y: 2.0 / 5.0,
        z: 3.0 / 5.0,
    };
    assert_eq!(a / b, c);
}

#[test]
fn test_mult_vec_scalar() {
    let a = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let b = 5.0;
    let c = Vec3 {
        x: 1.0 * 5.0,
        y: 2.0 * 5.0,
        z: 3.0 * 5.0,
    };
    assert_eq!(a * b, c);
}

#[test]
fn test_dot() {
    let a = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let b = Vec3 {
        x: 4.0,
        y: 5.0,
        z: 6.0,
    };
    let c = 4.0 + 10.0 + 18.0;
    assert_eq!(a.dot(b), c);
}

#[test]
fn test_cross() {
    let a = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let b = Vec3 {
        x: 4.0,
        y: 5.0,
        z: 6.0,
    };
    let c = Vec3 {
        x: -3.0,
        y: 6.0,
        z: -3.0,
    };
    assert_eq!(a.cross(b), c);
}
