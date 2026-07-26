// [TRANSLATION_NOTE]: Rect.h -> Rust 泛型结构体
// C++ TRect 模板映射为 Rust 泛型 TRect，使用 std::cmp 替代 std::min/std::max

use std::cmp;
use std::ops::AddAssign;

use super::point::TPoint;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRect<T> {
    pub m_x: T,
    pub m_y: T,
    pub m_width: T,
    pub m_height: T,
}

impl<T> TRect<T> {
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        TRect {
            m_x: x,
            m_y: y,
            m_width: width,
            m_height: height,
        }
    }
}

impl<T> TRect<T>
where
    T: Default,
{
    pub fn default() -> Self {
        TRect {
            m_x: Default::default(),
            m_y: Default::default(),
            m_width: Default::default(),
            m_height: Default::default(),
        }
    }
}

impl<T: PartialOrd + Add<Output = T> + Copy + Default + Sub<Output = T> + Ord> TRect<T> {
    pub fn intersects(&self, other: &TRect<T>) -> bool {
        !((other.m_x + other.m_width <= self.m_x)
            || (other.m_y + other.m_height <= self.m_y)
            || (other.m_x >= self.m_x + self.m_width)
            || (other.m_y >= self.m_y + self.m_height))
    }

    pub fn intersection(&self, other: &TRect<T>) -> TRect<T> {
        let x1 = cmp::max(self.m_x, other.m_x);
        let x2 = cmp::min(self.m_x + self.m_width, other.m_x + other.m_width);
        let y1 = cmp::max(self.m_y, other.m_y);
        let y2 = cmp::min(self.m_y + self.m_height, other.m_y + other.m_height);
        if (x2 - x1 < T::default()) || (y2 - y1 < T::default()) {
            TRect::new(T::default(), T::default(), T::default(), T::default())
        } else {
            TRect::new(x1, y1, x2 - x1, y2 - y1)
        }
    }

    pub fn union(&self, other: &TRect<T>) -> TRect<T> {
        let x1 = cmp::min(self.m_x, other.m_x);
        let x2 = cmp::max(self.m_x + self.m_width, other.m_x + other.m_width);
        let y1 = cmp::min(self.m_y, other.m_y);
        let y2 = cmp::max(self.m_y + self.m_height, other.m_y + other.m_height);
        TRect::new(x1, y1, x2 - x1, y2 - y1)
    }
}

impl<T: PartialOrd + Add<Output = T> + Copy> TRect<T> {
    pub fn contains_point(&self, x: T, y: T) -> bool {
        (x >= self.m_x) && (x < self.m_x + self.m_width)
            && (y >= self.m_y) && (y < self.m_y + self.m_height)
    }

    pub fn contains(&self, point: &TPoint<T>) -> bool {
        (point.m_x >= self.m_x) && (point.m_x < self.m_x + self.m_width)
            && (point.m_y >= self.m_y) && (point.m_y < self.m_y + self.m_height)
    }
}

impl<T: AddAssign + Copy> TRect<T> {
    pub fn offset(&mut self, x: T, y: T) {
        self.m_x += x;
        self.m_y += y;
    }

    pub fn offset_point(&mut self, point: &TPoint<T>) {
        self.m_x += point.m_x;
        self.m_y += point.m_y;
    }
}

impl<T: AddAssign + Sub<Output = T> + Add<Output = T> + Copy> TRect<T> {
    pub fn inflate(&mut self, x: T, y: T) -> TRect<T>
    where
        T: Add<Output = T>,
    {
        self.m_x = self.m_x - x;
        self.m_width = self.m_width + x + x;
        self.m_y = self.m_y - y;
        self.m_height = self.m_height + y + y;
        *self
    }
}

use std::ops::{Add, Sub};

pub type Rect = TRect<i32>;
pub type FRect = TRect<f64>;
