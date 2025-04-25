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
#[derive(Debug, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

// Matrix impl with new, zero, identity
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
