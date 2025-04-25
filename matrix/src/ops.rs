use std::ops::{Add, Sub};
use crate::{Matrix, Scalar};

impl<T: Scalar<Item = T> + Clone + Add<Output = T>> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0[0].len() != rhs.0[0].len() {
            return None;
        }

        let data = self.0.into_iter().zip(rhs.0)
            .map(|(r1, r2)| r1.into_iter().zip(r2).map(|(a, b)| a + b).collect())
            .collect();

        Some(Matrix(data))
    }
}

impl<T: Scalar<Item = T> + Clone + Sub<Output = T>> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0[0].len() != rhs.0[0].len() {
            return None;
        }

        let data = self.0.into_iter().zip(rhs.0)
            .map(|(r1, r2)| r1.into_iter().zip(r2).map(|(a, b)| a - b).collect())
            .collect();

        Some(Matrix(data))
    }
}
