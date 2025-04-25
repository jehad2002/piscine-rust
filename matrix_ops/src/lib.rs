// src/lib.rs

pub mod ops; // This makes the ops module public
pub use ops::*; // This makes the functions and implementations from ops.rs available here.

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        Matrix(matrix)
    }
}
