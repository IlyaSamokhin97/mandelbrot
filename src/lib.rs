use std::iter::successors;

pub use num::complex::Complex64;

pub fn mandelbrot(c: Complex64, precision: usize) -> usize {
    successors(Some(c), |z| Some(z*z + c))
        .enumerate()
        .take(precision)
        .find(|(_, z)| z.norm() > 2.0)
        .map(|(i, _)| i)
        .unwrap_or(precision)
}