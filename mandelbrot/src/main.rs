use image::codecs::png::{self, PngEncoder};
use image::{ColorType, GrayImage, ImageEncoder};
use num::Complex;
use std::env;
use std::f64::INFINITY;
use std::fs::File;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimentions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left coner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right coner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];
    // render(&mut pixels, bounds, upper_left, lower_right);
    let threads = 8;
    // let threads = 8 * 2;
    let rows_per_band = bounds.1 / threads + 1;
    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);

                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        })
        .unwrap();
    }

    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    // let output = File::create(filename)?;
    let output = match File::create(filename) {
        Ok(f) => f,
        Err(e) => {
            return Err(e);
        }
    };

    let encoder = PngEncoder::new(output);
    encoder.write_image(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::L8);

    Ok(())
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte. the `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper-
/// left and lower-right corners of the pixel buffer.
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);

            // bounds.0 is width
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

/// Given the row and column of a pixel in the output image, return the
/// corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and height of image in pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex
/// plane designating the area our image covers.
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
        // Why substraction here? pixel.1 increases aswe go down,
        // but the imaginary component increases as we go up.
    }
}

/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
///
/// Specifically, `s` sould have the form <left><sep><right>, where <sep> is
/// the character given by the `separator` argument, and <left> and <right> are both
/// strings that can be parsed by `T::from_str`.
/// `separator` must be an ASCII character.

/// If `s` has the proper form, return `Some<(x, y)>`. If it doesn't parse
/// correctly, return `None`.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        // slice
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

/// Parse a pair of floating-point numbers separated by a comma as a complex number.
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

/// Try to determine if `c` is in the Mandelbrot set, using at most `limit` iterations to decide.
///
/// if `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius two centered on the
/// origin. If `c` seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that `c` is not a member),
/// return `None`
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c
    }
    None
}

fn complex_square_ad_loop(c: Complex<f64>, t: u64) -> Complex<f64> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let mut n = 0;
    // loop {
    while n < t {
        z = z * z + c;
        n += 1
    }
    z
}

fn square_loop(mut x: f64, t: u64) -> f64 {
    let mut n = 0;
    // loop {
    while n < t {
        x = x * x;
        n += 1;
    }
    x
}

fn square_add_loop(c: f64, t: u64) -> f64 {
    let mut x = 0.;
    let mut n = 0;
    // loop {
    while n < t {
        x = x * x + c;
        n += 1;
    }
    x
}

#[test]
fn test_square_loop() {
    assert_eq!(square_loop(1.0, 1), 1.0);
    assert_eq!(square_loop(1.0, 10), 1.0);
    assert_eq!(square_loop(1.0, 100), 1.0);

    assert_eq!(square_loop(1.01, 1), 1.0201);
    assert_eq!(square_loop(1.01, 10), 26612.566117305338);
    assert_eq!(square_loop(1.01, 100), INFINITY);

    assert_eq!(square_loop(1.1, 1), 1.2100000000000002);
    assert_eq!(square_loop(1.1, 10), 2.432817896953651e42);
    assert_eq!(square_loop(1.1, 100), INFINITY);

    assert_eq!(square_loop(2.0, 1), 4.0);
    assert_eq!(square_loop(2.0, 10), INFINITY);
    assert_eq!(square_loop(2.0, 100), INFINITY);
}

#[test]
fn test_square_add_loop() {
    assert_eq!(square_add_loop(0.25, 1), 0.25);
    assert_eq!(square_add_loop(0.25, 10), 0.430549106102856);
    assert_eq!(square_add_loop(0.25, 100), 0.4906042201293854);
    assert_eq!(square_add_loop(0.25, 100000), 0.49999000132792415);

    assert_eq!(square_add_loop(0.251, 100), 476.37172174929185);
    assert_eq!(square_add_loop(0.251, 1000), INFINITY);
}

#[test]
fn test_complex_square_ad_loop() {
    assert_eq!(
        complex_square_ad_loop(Complex { re: 1.0, im: 1.0 }, 1).norm(),
        1.4142135623730951
    );
    assert_eq!(
        complex_square_ad_loop(Complex { re: 1.0, im: 1.0 }, 10).norm(),
        1.4235116073289224e127
    );
    assert!(complex_square_ad_loop(Complex { re: 1.0, im: 1.0 }, 100)
        .norm()
        .is_nan());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_time() {
        // Test for a point that is clearly in the set
        let c_in_set = Complex { re: 0.0, im: 0.0 };
        assert_eq!(escape_time(c_in_set, 1000), None);

        // Test for a point that is clearly not in the set
        let c_not_in_set = Complex { re: 2.0, im: 2.0 };
        assert_eq!(escape_time(c_not_in_set, 10), Some(1));
        assert_eq!(escape_time(c_not_in_set, 10000), Some(1));

        // Test for a point on the edge of the set
        let c_on_edge = Complex { re: -1.0, im: 0.0 };
        assert_eq!(escape_time(c_on_edge, 1000), None);

        // Test the limit argument
        let c = Complex { re: 0.0, im: 1.0 };
        assert_eq!(escape_time(c, 1), None);
        assert_eq!(escape_time(c, 2), None);

        let c = Complex { re: 0.0, im: 1.0 };
        assert_eq!(escape_time(c, 1), None);
    }

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair::<i32>("400x600", 'x'), Some((400, 600)));
        assert_eq!(parse_pair::<i32>("-400x600", 'x'), Some((-400, 600)));
        assert_eq!(parse_pair::<i32>("400x-600", 'x'), Some((400, -600)));
        assert_eq!(parse_pair::<i32>("400,600", ','), Some((400, 600)));

        assert_eq!(parse_pair::<f32>("1.0,0.5", ','), Some((1.0, 0.5)));
        assert_eq!(parse_pair::<f32>("-1.0,-0.5", ','), Some((-1.0, -0.5)));
        assert_eq!(parse_pair::<f32>("1.0 0.5", ' '), Some((1.0, 0.5)));

        assert_eq!(parse_pair::<f32>("1.0 0.5", ','), None);
        assert_eq!(parse_pair::<f32>("1.0,0.5", ' '), None);

        assert_eq!(parse_pair::<f32>("abc,def", ','), None);
        assert_eq!(parse_pair::<f32>("abc def", ' '), None);

        assert_eq!(parse_pair::<i32>("", 'x'), None);
        assert_eq!(parse_pair::<f32>("", ','), None);
        assert_eq!(parse_pair::<i32>("x", 'x'), None);
        assert_eq!(parse_pair::<f32>(",", ','), None);
    }

    #[test]
    fn test_parse_complex() {
        assert_eq!(
            parse_complex("1.25,-0.00625"),
            Some(Complex {
                re: 1.25,
                im: -0.00625
            })
        );

        assert_eq!(parse_complex(",-0.0625"), None);
    }

    #[test]
    fn test_pixel_to_point() {
        assert_eq!(
            pixel_to_point(
                (100, 200),
                (25, 175),
                Complex { re: -1.0, im: 1.0 },
                Complex { re: 1.0, im: -1.0 }
            ),
            Complex {
                re: -0.5,
                im: -0.75
            }
        );
    }

    #[test]
    fn test_render() {
        {
            let mut pixels = vec![0; 0];

            render(
                &mut pixels,
                (0, 0),
                Complex::new(-1.0, 1.0),
                Complex::new(1.0, -1.0),
            );

            assert_eq!(pixels, vec![0; 0]);
        }
        {
            let mut pixels = vec![0; 9];

            render(
                &mut pixels,
                (3, 3),
                Complex::new(-1.0, 1.0),
                Complex::new(1.0, -1.0),
            );

            assert_eq!(pixels, vec![252, 250, 252, 244, 0, 0, 244, 0, 0]);
        }
    }
}
