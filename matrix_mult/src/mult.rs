// matrix/src/mult.rs
use std::ops::Mul;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Clone> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() || self.0[0].is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n].clone()).collect()
    }
}

impl<T> Mul for Matrix<T>
where
    T: Copy + Mul<Output = T> + std::iter::Sum + Default + Debug,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.number_of_cols() != rhs.number_of_rows() {
            return None;
        }

        let rows = self.number_of_rows();
        let cols = rhs.number_of_cols();
        let common_dim = self.number_of_cols();

        let mut result = vec![vec![T::default(); cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                result[i][j] = (0..common_dim)
                    .map(|k| self.0[i][k] * rhs.0[k][j])
                    .sum();
            }
        }

        Some(Matrix(result))
    }
}
