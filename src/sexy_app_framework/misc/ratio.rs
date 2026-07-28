// [TRANSLATION_NOTE]: Ratio.h + Ratio.cpp -> Rust
// 比例计算工具，使用欧几里得算法化简分数

use std::ops::{Mul, Div};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ratio {
    pub m_numerator: i32,
    pub m_denominator: i32,
}

impl Ratio {
    pub fn new() -> Self {
        Ratio { m_numerator: 1, m_denominator: 1 }
    }

    pub fn with_values(the_numerator: i32, the_denominator: i32) -> Self {
        let mut r = Ratio { m_numerator: 1, m_denominator: 1 };
        r.set(the_numerator, the_denominator);
        r
    }

    pub fn set(&mut self, the_numerator: i32, the_denominator: i32) {
        // find the greatest-common-denominator of theNumerator and theDenominator.
        let mut a = the_numerator;
        let mut b = the_denominator;
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        // divide by the g-c-d to reduce to lowest terms.
        self.m_numerator = the_numerator / a;
        self.m_denominator = the_denominator / a;
    }
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.m_denominator == 0 || other.m_denominator == 0 {
            return None;
        }
        let a = self.m_numerator * other.m_denominator / self.m_denominator;
        let b = other.m_numerator;
        Some(a.cmp(&b))
    }
}

impl Mul<i32> for Ratio {
    type Output = i32;
    fn mul(self, the_int: i32) -> i32 {
        the_int * self.m_numerator / self.m_denominator
    }
}

impl Mul<Ratio> for i32 {
    type Output = i32;
    fn mul(self, the_ratio: Ratio) -> i32 {
        self * the_ratio.m_numerator / the_ratio.m_denominator
    }
}

impl Div<i32> for Ratio {
    type Output = i32;
    fn div(self, the_int: i32) -> i32 {
        the_int * self.m_denominator / self.m_numerator
    }
}

impl Div<Ratio> for i32 {
    type Output = i32;
    fn div(self, the_ratio: Ratio) -> i32 {
        self * the_ratio.m_denominator / the_ratio.m_numerator
    }
}
