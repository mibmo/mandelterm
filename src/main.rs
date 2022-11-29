pub mod complex;
use complex::*;
pub mod view;
use view::*;

#[inline(always)]
fn has_exploded(p: C32) -> bool {
    p.distance(complex::ORIGIN) > 2.0
}

fn main() {
    let mut view = View::new(80, 40);

    for x in 0..view.width {
        for y in 0..view.height {
            let mut fill = false;
            let p: C32 = (x as f32 / 30.0, y as f32 / 15.0).into();

            /*
            if has_exploded(p) {
                view.set(x, y, true);
            }
            */

            /* // this works correctly
            if x % 2 == 1 {
                fill = true;
            }
            */

            /* // this highlights a mistake i need to fix in the Display code
            if y % 2 == 1 {
                fill = true;
            }
            */

            view.set(x, y, fill);
            // mandelbrot check
        }
    }

    println!("{view}");
}
