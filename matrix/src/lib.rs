use std::ops::Mul;
use std::fmt::Debug;

// Define the Scalar trait
pub trait Scalar: Clone {
    fn zero() -> Self;
    fn one() -> Self;
}

// Implement Scalar for commonly used types
impl Scalar for i32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for u32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for f64 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
}

// Matrix struct
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

// Implement basic constructors
impl<T: Scalar> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut mat = vec![vec![T::zero(); n]; n];
        for i in 0..n {
            mat[i][i] = T::one();
        }
        Matrix(mat)
    }
}

// Matrix utility methods
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

// Matrix multiplication
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
        let common = self.number_of_cols();

        let mut result = vec![vec![T::default(); cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                result[i][j] = (0..common)
                    .map(|k| self.0[i][k] * rhs.0[k][j])
                    .sum();
            }
        }

        Some(Matrix(result))
    }
}
