// src/ops.rs

use crate::Matrix;
use std::ops::{Add, Sub};

impl Add for Matrix {
    type Output = Option<Matrix>;

    fn add(self, other: Matrix) -> Option<Matrix> {
        let rows_self = self.0.len();
        let cols_self = self.0.get(0).map(|row| row.len()).unwrap_or(0);
        let rows_other = other.0.len();
        let cols_other = other.0.get(0).map(|row| row.len()).unwrap_or(0);

        // Matrices must have the same dimensions
        if rows_self != rows_other || cols_self != cols_other {
            return None;
        }

        let mut result = Vec::new();

        for i in 0..rows_self {
            let mut row = Vec::new();
            for j in 0..cols_self {
                row.push(self.0[i][j] + other.0[i][j]);
            }
            result.push(row);
        }

        Some(Matrix(result))
    }
}

impl Sub for Matrix {
    type Output = Option<Matrix>;

    fn sub(self, other: Matrix) -> Option<Matrix> {
        let rows_self = self.0.len();
        let cols_self = self.0.get(0).map(|row| row.len()).unwrap_or(0);
        let rows_other = other.0.len();
        let cols_other = other.0.get(0).map(|row| row.len()).unwrap_or(0);

        // Matrices must have the same dimensions
        if rows_self != rows_other || cols_self != cols_other {
            return None;
        }

        let mut result = Vec::new();

        for i in 0..rows_self {
            let mut row = Vec::new();
            for j in 0..cols_self {
                row.push(self.0[i][j] - other.0[i][j]);
            }
            result.push(row);
        }

        Some(Matrix(result))
    }
}
