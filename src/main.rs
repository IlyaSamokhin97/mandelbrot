use mandelbrot::*;

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
        let precisionf: f64 = num::cast(precision).expect("precision should fit float");

        if let Some(mov) = move_vector(input) {
            self.center += mov * dt;
        }

        dbg!(dt.recip());
        dbg!(rayon::current_num_threads());

        let height = raw_canvas.height();
        let width = raw_canvas.width();
        let widthf = num::cast(width).expect("width should fit float");

        let changes = (0..height).into_par_iter().flat_map(|y| (0..width).into_par_iter().map(move |x| (y, x)))
            .map(|(y, x)| {
                let xf = num::cast(x).expect("x should fit float");
                let yf = num::cast(y).expect("y should fit float");

                let c = Complex64::new(
                    range_map(xf, [0.0, widthf], [self.center.re - 1.0, self.center.re + 1.0]),
                    range_map(yf, [0.0, widthf], [self.center.im - 1.0, self.center.im + 1.0]),
                );

                let result = mandelbrot(c, precision);
                let color: u32 = {
                    let resultf = num::cast(result).expect("result is not bigget than precision");
                    let v: u32 = num::cast(range_map(resultf, [0.0, precisionf], [0.0, 255.0]))
                        .expect("float in [0.0, 255.0] range should fit u32");

                    0xFF000000
                        | v << 16
                        | v << 8
                        | v
                };
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
    let l = input.keyboard.left.is_pressed();
    let r = input.keyboard.right.is_pressed();
    let u = input.keyboard.up.is_pressed();
    let d = input.keyboard.down.is_pressed();

    match (l, r, u, d) {
        (true , false, true , false) => Some(Complex64::new(-1.0,  1.0)),
        (false, false, true , false) => Some(Complex64::new( 0.0,  1.0)),
        (false, true , true , false) => Some(Complex64::new( 1.0,  1.0)),
        (false, true , false, false) => Some(Complex64::new( 1.0,  0.0)),
        (false, true , false, true ) => Some(Complex64::new( 1.0, -1.0)),
        (false, false, false, true ) => Some(Complex64::new( 0.0, -1.0)),
        (true , false, false, true ) => Some(Complex64::new(-1.0, -1.0)),
        (true , false, false, false) => Some(Complex64::new(-1.0,  0.0)),
        _ => None,
    }
}