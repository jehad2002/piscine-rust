pub mod ops;

pub trait Scalar {
    type Item;

    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

impl Scalar for i32 {
    type Item = i32;
    fn zero() -> Self::Item { 0 }
    fn one() -> Self::Item { 1 }
}

impl Scalar for u32 {
    type Item = u32;
    fn zero() -> Self::Item { 0 }
    fn one() -> Self::Item { 1 }
}

impl Scalar for f64 {
    type Item = f64;
    fn zero() -> Self::Item { 0.0 }
    fn one() -> Self::Item { 1.0 }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T> + Clone> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::one()]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut result = vec![vec![T::zero(); n]; n];
        for i in 0..n {
            result[i][i] = T::one();
        }
        Matrix(result)
    }
}
