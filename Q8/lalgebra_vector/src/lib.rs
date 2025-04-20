use std::ops::{Add, Mul};
use std::fmt::Debug;

pub trait Scalar: Copy + Add<Output = Self> + Mul<Output = Self> + Debug + PartialEq + Eq {}
impl<T> Scalar for T where T: Copy + Add<Output = T> + Mul<Output = T> + Debug + PartialEq + Eq {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }

        let result = self.0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| *a + *b)
            .collect();

        Some(Vector(result))
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut iter = self.0.iter().zip(&other.0);
        let (first, second) = iter.next()?;
        let mut result = *first * *second;

        for (a, b) in iter {
            result = result + (*a * *b);
        }

        Some(result)
    }
}
