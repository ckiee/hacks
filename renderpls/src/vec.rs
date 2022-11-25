use std::ops::{Add, Sub, Mul};

use bmp::Pixel;
#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub struct Vec2(pub i64, pub i64);

pub type Vec1 = i64;
pub type Painter<V> = fn(V) -> Pixel;
pub type Painter2 = Painter<Vec2>;

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Vec2 {
    /// Swaps between X and Y in the Vec.
    pub fn swap(self) -> Self {
        Self(self.1, self.0)
    }
}
