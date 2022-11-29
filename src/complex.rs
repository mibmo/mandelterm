use std::convert::From;
use std::fmt::{self, Debug, Display};
use std::ops::{Add, Mul, Sub};

pub const ORIGIN: Complex<f32> = Complex { real: 0.0, imaginary: 0.0 };

/// A 32-bit complex number.
pub type C32 = Complex<f32>;
/// A 64-bit complex number.
pub type C64 = Complex<f64>;

/// Partial implementation of Complex numbers.
///
/// Only the operations needed for recreating the mandelbrot
/// set are implemented, which essentially boils down to
/// addition, subtraction, multiplication, and (integer) exponentation.
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Complex<T> {
    real: T,
    imaginary: T,
}

impl<T: Add<Output = T>> Add for Complex<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Complex<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            imaginary: self.imaginary - rhs.imaginary,
        }
    }
}

impl<T: Mul<Output = T>> Mul for Complex<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real * rhs.real,
            imaginary: self.imaginary * rhs.imaginary,
        }
    }
}

impl<T: Copy + Mul<Output = T>> Complex<T> {
    pub fn powi(self, n: i32) -> Self {
        let mut real = self.real;
        let mut imaginary = self.imaginary;

        for _ in 0..n {
            real = real * real;
            imaginary = imaginary * imaginary;
        }

        Self { real, imaginary }
    }
}

// @TODO: consolidate f32/f64 specific impls into macro
impl Complex<f32> {
    pub fn abs(self) -> Self {
        Self {
            real: self.real.abs(),
            imaginary: self.imaginary.abs(),
        }
    }

    pub fn distance(self, other: Complex<f32>) -> f32 {
        let p = (self - other).abs();
        (p.real.powi(2) + p.imaginary.powi(2)).sqrt()
    }
}

impl<T: Display> Display for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let real = &self.real;
        let imaginary = &self.imaginary;
        write!(f, "{real} + {imaginary}i")
    }
}

impl<T: Display> Debug for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl From<(f32, f32)> for Complex<f32> {
    fn from((real, imaginary): (f32, f32)) -> Complex<f32> {
        Complex { real, imaginary }
    }
}

impl From<(f64, f64)> for Complex<f64> {
    fn from((real, imaginary): (f64, f64)) -> Complex<f64> {
        Complex { real, imaginary }
    }
}

impl From<(i32, i32)> for Complex<f32> {
    fn from((real, imaginary): (i32, i32)) -> Complex<f32> {
        Complex {
            real: real as f32,
            imaginary: imaginary as f32,
        }
    }
}

impl From<(i64, i64)> for Complex<f64> {
    fn from((real, imaginary): (i64, i64)) -> Complex<f64> {
        Complex {
            real: real as f64,
            imaginary: imaginary as f64,
        }
    }
}

#[test]
fn addition() {
    let a: Complex<f32> = (2.0, 5.0).into();
    let b: Complex<f32> = (7.0, 2.0).into();
    assert_eq!(a + b, (9.0, 7.0).into())
}

#[test]
fn subtraction() {
    let a: Complex<f32> = (12, 3).into();
    let b: Complex<f32> = (10, -5).into();
    assert_eq!(a - b, (2, 8).into())
}

#[test]
fn multiplication() {
    let a: Complex<f32> = (2.0, 5.0).into();
    let b: Complex<f32> = (7.0, 2.0).into();
    assert_eq!(a * b, (14.0, 10.0).into())
}

#[test]
fn powi() {
    let a: Complex<f32> = (2.0, 5.0).into();

    assert_eq!(a.powi(0), a);
    assert_eq!(a.powi(1), (4.0, 25.0).into());
    assert_eq!(a.powi(2), (16.0, 625.0).into());
    assert_eq!(a.powi(3), (256.0, 390625.0).into());
}

#[test]
fn distance() {
    let a: Complex<f32> = (12, 3).into();
    let b: Complex<f32> = (10, -5).into();

    assert_eq!(a.distance(b), 8.246211); // 8.246211 = 2√17
}
