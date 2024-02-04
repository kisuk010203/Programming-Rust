extern crate num;
use num::{complex::ParseComplexError, Complex};
use std::{path::Prefix, str::FromStr};

/// Parse the string `s` as a coordinate pair, like `"400X600"` or `"1.0,0.5"`.
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is the character given by the `separator` argument,
/// and <left> and <right> are both strings that can be parsed by `T::from_str`.
///
/// If `s` has the proper form, return `Some(<x, y>)`. Otherwise, return `None`.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}
#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.0", 'x'), Some((0.5, 1.0)));
}
/// Parse a pair of floating-point numbers separated by a comma as a complex number.
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair::<f64>(s, ',') {
        Some((l, r)) => Some(Complex { re: l, im: r }),
        _ => None,
    }
}
#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("0.5,1.2"), Some(Complex { re: 0.5, im: 1.2 }));
    assert_eq!(parse_complex("1902312390h1bnkln"), None);
}
/// Given the row and column of a pixel in the output image, return the corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels.
/// `pixel` is a (column, row) pair that indicates a particular pixel in that range.
/// The `upper_left` and `lower_right` parameters are points on the complex plane designating the area of our image covers.
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);
    let x = Complex{re: 2.1, im:3.2};
    x
}
fn square_loop(mut x: i64) {
    loop {
        // This is an infinite loop.
        x = x * x;
    }
}
fn square_add_loop(c: f64) {
    let mut x = 0.;
    loop {
        // This is another infinite loop.
        x = x * x + c;
    }
}
#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    // `Complex` is a generic structure.
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}
/// Try to determine if `c` is in the Mandelbrot set, using at most `limit` iterations to decide.
/// If `c` is not a member, return `Some(i)`, where `i` is the number of iterations it took for `c`
/// to leave the circle of radius two centered on the origin. If `c` seems to be a member, return `None`.
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}
fn main() {
    let c = Complex { re: 0.1, im: 0.3 };
    let limit = 100;
    let escape = escape_time(c, limit);
    println!("Is {:?} a Mandelbrot number? {:?}", c, escape);
}
