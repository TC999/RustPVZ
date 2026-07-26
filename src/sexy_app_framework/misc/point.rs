// [TRANSLATION_NOTE]: Point.h -> Rust 泛型结构体
// C++ template 映射为 Rust 泛型，运算符重载映射为 std::ops trait 实现

use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TPoint<T> {
    pub m_x: T,
    pub m_y: T,
}

impl<T> TPoint<T> {
    pub fn new(x: T, y: T) -> Self {
        TPoint { m_x: x, m_y: y }
    }
}

impl<T> TPoint<T>
where
    T: Default,
{
    pub fn default() -> Self {
        TPoint {
            m_x: Default::default(),
            m_y: Default::default(),
        }
    }
}

impl<T: Add<Output = T>> Add for TPoint<T> {
    type Output = TPoint<T>;
    fn add(self, p: TPoint<T>) -> TPoint<T> {
        TPoint::new(self.m_x + p.m_x, self.m_y + p.m_y)
    }
}

impl<T: Sub<Output = T>> Sub for TPoint<T> {
    type Output = TPoint<T>;
    fn sub(self, p: TPoint<T>) -> TPoint<T> {
        TPoint::new(self.m_x - p.m_x, self.m_y - p.m_y)
    }
}

impl<T: Mul<Output = T>> Mul for TPoint<T> {
    type Output = TPoint<T>;
    fn mul(self, p: TPoint<T>) -> TPoint<T> {
        TPoint::new(self.m_x * p.m_x, self.m_y * p.m_y)
    }
}

impl<T: Div<Output = T>> Div for TPoint<T> {
    type Output = TPoint<T>;
    fn div(self, p: TPoint<T>) -> TPoint<T> {
        TPoint::new(self.m_x / p.m_x, self.m_y / p.m_y)
    }
}

impl<T: AddAssign> AddAssign for TPoint<T> {
    fn add_assign(&mut self, p: TPoint<T>) {
        self.m_x += p.m_x;
        self.m_y += p.m_y;
    }
}

impl<T: SubAssign> SubAssign for TPoint<T> {
    fn sub_assign(&mut self, p: TPoint<T>) {
        self.m_x -= p.m_x;
        self.m_y -= p.m_y;
    }
}

impl<T: MulAssign> MulAssign for TPoint<T> {
    fn mul_assign(&mut self, p: TPoint<T>) {
        self.m_x *= p.m_x;
        self.m_y *= p.m_y;
    }
}

impl<T: DivAssign> DivAssign for TPoint<T> {
    fn div_assign(&mut self, p: TPoint<T>) {
        self.m_x /= p.m_x;
        self.m_y /= p.m_y;
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for TPoint<T> {
    type Output = TPoint<T>;
    fn mul(self, s: T) -> TPoint<T> {
        TPoint::new(self.m_x * s, self.m_y * s)
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for TPoint<T> {
    type Output = TPoint<T>;
    fn div(self, s: T) -> TPoint<T> {
        TPoint::new(self.m_x / s, self.m_y / s)
    }
}

pub type Point = TPoint<i32>;
pub type FPoint = TPoint<f64>;
