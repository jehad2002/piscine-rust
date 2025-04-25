use std::ops::{Mul, AddAssign};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Clone> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() {
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
    T: Clone + Default + Mul<Output = T> + AddAssign,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        let rows_lhs = self.number_of_rows();
        let cols_lhs = self.number_of_cols();
        let rows_rhs = rhs.number_of_rows();
        let cols_rhs = rhs.number_of_cols();

        if cols_lhs != rows_rhs {
            return None;
        }

        let mut result = vec![vec![T::default(); cols_rhs]; rows_lhs];

        for i in 0..rows_lhs {
            for j in 0..cols_rhs {
                for k in 0..cols_lhs {
                    result[i][j] += self.0[i][k].clone() * rhs.0[k][j].clone();
                }
            }
        }

        Some(Matrix(result))
    }
}
