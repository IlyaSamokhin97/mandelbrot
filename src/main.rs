use std::iter::successors;

use num::complex::Complex64;
use rayon::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut data = Data::default();
    lume::run(&mut data)
}

#[derive(Default)]
struct Data {
    center: Complex64,
}

impl lume::Data for Data {
    fn update(&mut self, raw_canvas: &mut dyn lume::RawCanvas, input: &lume::Input, dt: f64) {
        let precision = 100;

        if let Some(mov) = move_vector(input) {
            self.center += mov * dt;
        }

        dbg!(dt.recip());
        dbg!(rayon::current_num_threads());

        let height = raw_canvas.height();
        let width = raw_canvas.width();

        let changes = (0..height).into_par_iter().flat_map(|y| (0..width).into_par_iter().map(move |x| (y, x)))
            .map(|(y, x)| {
                let xf: f64 = x as _;
                let yf: f64 = y as _;

                let c = Complex64::new(
                    range_map(xf, [0.0, width as _], [self.center.re - 1.0, self.center.re + 1.0]),
                    range_map(yf, [0.0, width as _], [self.center.im - 1.0, self.center.im + 1.0]),
                );

                let result = mandelbrot(c, precision);
                let color = iterations_to_color(result, precision);
                (y*width + x, color)
            })
            .collect::<Vec<_>>();
        for (i, c) in changes {
            raw_canvas[i] = c;
        }
    }
}

fn range_map(n: f64, [sb, se]: [f64; 2], [db, de]: [f64; 2]) -> f64 {
    let scale = (de - db) / (se - sb);
    (n - sb) * scale + db
}

fn move_vector(input: &lume::Input) -> Option<Complex64> {
    let kb = &input.keyboard;

    [
        (&kb.left , (-1.0,  0.0)),
        (&kb.right, ( 1.0,  0.0)),
        (&kb.up   , ( 0.0,  1.0)),
        (&kb.down , ( 0.0, -1.0)),
    ]
        .into_iter()
        .filter_map(|(btn, (re, im))| btn.is_pressed().then(|| Complex64::new(re, im)))
        .fold(None, |acc, x| Some(x + acc.unwrap_or_else(num::zero)))
}

fn mandelbrot(c: Complex64, precision: usize) -> usize {
    successors(Some(c), |z| Some(z*z + c))
        .enumerate()
        .take(precision)
        .find(|(_, z)| z.norm_sqr() > 4.0)
        .map_or(precision, |(i, _)| i)
}

fn iterations_to_color(iterations: usize, precision: usize) -> u32 {
    let v = range_map(iterations as _, [0.0, precision as _], [0.0, 255.0]) as u32;

    0xFF000000
        | v << 16
        | v << 8
        | v
}