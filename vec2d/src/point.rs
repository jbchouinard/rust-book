use std::{clone::Clone, cmp, fmt::Debug, marker::Copy, ops};

use crate::geom;

#[derive(Debug, Copy, Clone, cmp::PartialEq)]
pub struct Point<T>(pub T, pub T);

impl<T> ops::Mul<T> for Point<T>
where
    T: ops::Mul + Copy,
{
    type Output = Point<<T as ops::Mul>::Output>;

    fn mul(self, scalar: T) -> Self::Output {
        Point(self.0 * scalar, self.1 * scalar)
    }
}

impl<T> geom::Area for Point<T>
where
    T: ops::Mul + Copy,
{
    type Output = <T as ops::Mul>::Output;

    fn area(self) -> Self::Output {
        self.0 * self.1
    }
}

impl Point<f64> {
    pub fn magnitude(self) -> f64 {
        (self.0.powi(2) + self.1.powi(2)).sqrt()
    }
}

impl<T> ops::Add for Point<T>
where
    T: ops::Add + Copy,
{
    type Output = Point<<T as ops::Add>::Output>;

    fn add(self, other: Self) -> Self::Output {
        Point(self.0 + other.0, self.1 + other.1)
    }
}
