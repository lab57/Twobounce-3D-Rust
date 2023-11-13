use std::fmt;
use std::ops::Div;
use std::ops::{Add, Index, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    //v: Vec<f32>, //vector form of Vector
}
#[derive(Copy, Clone, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
impl Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, scalar: f32) -> Vector2 {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        return Vector { x, y, z };
    }
    pub fn norm(&self) -> Vector {
        *self / self.abs()
    }
    pub fn abs(&self) -> f32 {
        return (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt();
    }

    pub fn cross(&self, other: Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn dot(&self, other: Vector) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn calc_coord(&self, st: Vector, t: f32) -> Vector {
        return st + *self * t;
    }

    pub fn clac_t(&self, st: Vector, end_pt: Vector) -> f32 {
        return (end_pt - st).x / self.x;
    }

    pub fn rotate(&self, axis: Vector, angle: f32) -> Vector {
        return *self * angle.cos()
            + axis.cross(*self) * angle.sin()
            + axis * (axis.dot(*self) * (1.0 - angle.cos()));
    }
    pub fn getArr(&self) -> [f32; 3] {
        return [self.x, self.y, self.z];
    }
    pub fn angle_with(&self, other: Vector) -> f32 {
        let dot_product = self.dot(other);
        let magnitudes = self.abs() * other.abs();
        (dot_product / magnitudes).acos()
    }
}
impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, scalar: f32) -> Vector {
        Vector {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;
    fn mul(self, scalar: f32) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<Vector2> for Vector2 {
    type Output = Vector2;
    fn add(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}
fn main() {
    let v1 = Vector {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    let o = Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    println!("{} {}, {}", v1, v1.norm(), v1 * 2.0);
    println!("{}", v1.calc_coord(o, 0.5));
}
