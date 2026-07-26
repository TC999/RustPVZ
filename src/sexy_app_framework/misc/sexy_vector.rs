// [TRANSLATION_NOTE]: SexyVector.h -> Rust struct
// C++ 类映射为 Rust 结构体 + impl 块，运算符重载使用 std::ops trait

use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SexyVector2 {
    pub x: f32,
    pub y: f32,
}

impl SexyVector2 {
    pub fn new() -> Self {
        SexyVector2 { x: 0.0, y: 0.0 }
    }

    pub fn new_xy(x: f32, y: f32) -> Self {
        SexyVector2 { x, y }
    }

    pub fn dot(&self, v: &SexyVector2) -> f32 {
        self.x * v.x + self.y * v.y
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn normalize(&self) -> SexyVector2 {
        let a_mag = self.magnitude();
        if a_mag != 0.0 {
            *self / a_mag
        } else {
            *self
        }
    }

    pub fn perp(&self) -> SexyVector2 {
        SexyVector2::new_xy(-self.y, self.x)
    }
}

impl Default for SexyVector2 {
    fn default() -> Self {
        SexyVector2::new()
    }
}

impl Add for SexyVector2 {
    type Output = SexyVector2;
    fn add(self, v: SexyVector2) -> SexyVector2 {
        SexyVector2::new_xy(self.x + v.x, self.y + v.y)
    }
}

impl Sub for SexyVector2 {
    type Output = SexyVector2;
    fn sub(self, v: SexyVector2) -> SexyVector2 {
        SexyVector2::new_xy(self.x - v.x, self.y - v.y)
    }
}

impl Neg for SexyVector2 {
    type Output = SexyVector2;
    fn neg(self) -> SexyVector2 {
        SexyVector2::new_xy(-self.x, -self.y)
    }
}

impl Mul<f32> for SexyVector2 {
    type Output = SexyVector2;
    fn mul(self, t: f32) -> SexyVector2 {
        SexyVector2::new_xy(t * self.x, t * self.y)
    }
}

impl Div<f32> for SexyVector2 {
    type Output = SexyVector2;
    fn div(self, t: f32) -> SexyVector2 {
        SexyVector2::new_xy(self.x / t, self.y / t)
    }
}

impl AddAssign for SexyVector2 {
    fn add_assign(&mut self, v: SexyVector2) {
        self.x += v.x;
        self.y += v.y;
    }
}

impl SubAssign for SexyVector2 {
    fn sub_assign(&mut self, v: SexyVector2) {
        self.x -= v.x;
        self.y -= v.y;
    }
}

impl MulAssign<f32> for SexyVector2 {
    fn mul_assign(&mut self, t: f32) {
        self.x *= t;
        self.y *= t;
    }
}

impl DivAssign<f32> for SexyVector2 {
    fn div_assign(&mut self, t: f32) {
        self.x /= t;
        self.y /= t;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SexyVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl SexyVector3 {
    pub fn new() -> Self {
        SexyVector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new_xyz(x: f32, y: f32, z: f32) -> Self {
        SexyVector3 { x, y, z }
    }

    pub fn dot(&self, v: &SexyVector3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(&self, v: &SexyVector3) -> SexyVector3 {
        SexyVector3::new_xyz(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> SexyVector3 {
        let a_mag = self.magnitude();
        if a_mag != 0.0 {
            *self / a_mag
        } else {
            *self
        }
    }
}

impl Default for SexyVector3 {
    fn default() -> Self {
        SexyVector3::new()
    }
}

impl Add for SexyVector3 {
    type Output = SexyVector3;
    fn add(self, v: SexyVector3) -> SexyVector3 {
        SexyVector3::new_xyz(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl Sub for SexyVector3 {
    type Output = SexyVector3;
    fn sub(self, v: SexyVector3) -> SexyVector3 {
        SexyVector3::new_xyz(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl Mul<f32> for SexyVector3 {
    type Output = SexyVector3;
    fn mul(self, t: f32) -> SexyVector3 {
        SexyVector3::new_xyz(t * self.x, t * self.y, t * self.z)
    }
}

impl Div<f32> for SexyVector3 {
    type Output = SexyVector3;
    fn div(self, t: f32) -> SexyVector3 {
        SexyVector3::new_xyz(self.x / t, self.y / t, self.z / t)
    }
}
