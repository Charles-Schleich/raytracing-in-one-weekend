use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

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

    pub fn cross(mut self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(mut self) -> f64 {
        f64::sqrt(self.len_sqred())
    }

    pub fn len(mut self) -> f64 {
        f64::sqrt(self.len_sqred())
    }

    pub fn len_sqred(self) -> f64 {
        return &self.x * &self.x + &self.y * &self.y + &self.z * &self.z;
    }

    pub fn write_colour(self) {
        let i_r = (256.0*self.x).round() as u8;
        let i_g = (256.0*self.y).round() as u8;
        let i_b = (256.0*self.z).round() as u8;
        print!("{} {} {}\n", i_r, i_g, i_b)
    }
}

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
