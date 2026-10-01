//! Shape verification routines for matrix algebra, slicing, and broadcasting.

/// Helper checks to guard matrix and vector dimensional invariants.
pub struct ShapeEngine;

impl ShapeEngine {
    pub fn validate_slice_range(start: usize, end: usize) {
        if start >= end {
            panic!("[SHAPE ERROR] Invalid slice range: {}..{}", start, end);
        }
    }

    pub fn validate_matrix_slice_range(r_start: usize, r_end: usize, c_start: usize, c_end: usize) {
        if r_start >= r_end || c_start >= c_end {
            panic!("[SHAPE ERROR] Invalid matrix slice range");
        }
    }

    pub fn assert_matmul_dimensions(c1: usize, r2: usize, r1: usize, c2: usize) {
        if c1 != r2 {
            panic!(
                "[SHAPE ERROR] Matrix dimensions mismatch for multiplication: ({},{}) vs ({},{})",
                r1, c1, r2, c2
            );
        }
    }
}