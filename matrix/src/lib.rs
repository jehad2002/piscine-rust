// src/lib.rs
use std::ops::{Add, Sub};

#[derive(Debug, Clone)]
pub struct Matrix(pub Vec<Vec<i32>>);

impl Add for Matrix {
    type Output = Option<Matrix>;

    fn add(self, other: Matrix) -> Option<Matrix> {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }

        let result = self.0.iter()
            .zip(other.0.iter())
            .map(|(row1, row2)| {
                row1.iter()
                    .zip(row2.iter())
                    .map(|(a, b)| a + b)
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        Some(Matrix(result))
    }
}

impl Sub for Matrix {
    type Output = Option<Matrix>;

    fn sub(self, other: Matrix) -> Option<Matrix> {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }

        let result = self.0.iter()
            .zip(other.0.iter())
            .map(|(row1, row2)| {
                row1.iter()
                    .zip(row2.iter())
                    .map(|(a, b)| a - b)
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        Some(Matrix(result))
    }
}
