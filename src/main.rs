use std::iter::successors;
use std::num::Saturating;

use itertools::Itertools;
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

        self.center += move_vector(input) * dt;

        let height = raw_canvas.height();
        let width = raw_canvas.width();

        let changes: Vec<_> = (0..height).cartesian_product(0..width).collect_vec()
            .into_par_iter()
            .map(|(y, x)| {
                let c = screen_to_complex((y, x), width, self);

                let result = mandelbrot(c, precision);
                let color = iterations_to_color(result, precision);
                (y*width + x, color)
            })
            .collect();
        for (i, c) in changes {
            raw_canvas[i] = c;
        }

        let [y, x] = [input.mouse.y, input.mouse.x].map(Saturating);
        let radius = Saturating(5);

        line(raw_canvas, [[y         , x - radius], [y         , x + radius]]);
        line(raw_canvas, [[y - radius, x         ], [y + radius, x         ]]);
    }
}

fn line(canvas: &mut dyn lume::RawCanvas, ps: [[Saturating<usize>; 2]; 2]) {
    fn line_impl(canvas: &mut dyn lume::RawCanvas, ps: [[usize; 2]; 2]) {
        let ps = if ps[0][1] > ps[1][1] {
            [ps[1], ps[0]]
        } else {
            ps
        };

        let [[y0,x0], [y1,x1]] = ps;

        let height = canvas.height();
        let width = canvas.width();
        if x0 != x1 {
            // y = (y1 - y0)/(x1 - x0) * x + y0

            let slope = (y1 - y0) / (x1 - x0);
            for x in x0..=x1 {
                let y = slope * x + y0;
                if x < width && y < height {
                    let px = &mut canvas[y*width + x];
                    if *px == 0xFFFF_FFFF {
                        *px = 0xFF00_0000;
                    } else {
                        *px = 0xFFFF_FFFF;
                    }
                }
            }
        } else {
            let x = x0;
            for y in y0..=y1 {
                if x < width && y < height {
                    let px = &mut canvas[y*width + x];
                    if *px == 0xFFFF_FFFF {
                        *px = 0xFF00_0000;
                    } else {
                        *px = 0xFFFF_FFFF;
                    }
                }
            }
        }
    }

    line_impl(canvas, ps.map(|xs| xs.map(|x| x.0)))
}

fn screen_to_complex((y, x): (usize, usize), width: usize, data: &Data) -> Complex64 {
    Complex64::new(
        range_map(x as _, [0.0, width as _], [data.center.re - 1.0, data.center.re + 1.0]),
        range_map(y as _, [0.0, width as _], [data.center.im - 1.0, data.center.im + 1.0]),
    )
}

fn range_map(n: f64, [sb, se]: [f64; 2], [db, de]: [f64; 2]) -> f64 {
    let scale = (de - db) / (se - sb);
    (n - sb) * scale + db
}

fn move_vector(input: &lume::Input) -> Complex64 {
    let kb = &input.keyboard;

    [
        (&kb.left , (-1.0,  0.0)),
        (&kb.right, ( 1.0,  0.0)),
        (&kb.up   , ( 0.0, -1.0)),
        (&kb.down , ( 0.0,  1.0)),
    ]
        .into_iter()
        .filter_map(|(btn, (re, im))| btn.is_pressed().then(|| Complex64::new(re, im)))
        .sum()
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