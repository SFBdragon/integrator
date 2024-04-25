//! Numerically calculate integrals.
//! 
//! e.g. 
//! 
//! ```rust
//! #use std::f64::consts::*;
//! 
//! let result = int!(
//!     // integrand
//!     r;
//!     // integration bounds in the format
//!     // low_bound <expr>, high_bound <expr>, x <var>, dx <var> ,n <expr, optional>);
//!     // where (, n) is an optionally-specified resolution (default is 100)
//!     0.0, PI, t, dt, 1000;
//!     0.0, 2.0 * f64::sin(t), r, dr;
//!     0.0, f64::sqrt(4.0 - r * r), z, dz
//! );
//! ```

fn main() {
    use std::f64::consts::*;

    let result = int!(
        r;
        0.0, PI, t, dt;
        0.0, 2.0 * f64::sin(t), r, dr;
        0.0, f64::sqrt(4.0 - r * r), z, dz
    );

    println!("{}", result);
}


pub struct BoundIter {
    lo: f64,
    dx: f64,
    n: usize,
    i: usize,
}

impl BoundIter {
    pub fn new(lo: f64, hi: f64, n: usize) -> Self {
        Self { lo, dx: (hi - lo) / n as f64, n, i: 0 }
    }
}

impl Iterator for BoundIter {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.n {
            return None;
        } else {
            let x_mid_i = self.lo + (self.i as f64 + 0.5) * self.dx;
            self.i += 1;
            Some((x_mid_i, self.dx))
        }
    }
}

#[macro_export]
macro_rules! int {
    (@n) => ( 100 );
    (@n $n:expr) => ( $n );

    (@loop $total:ident; $exp:expr; $lo:expr, $hi:expr, $x:ident, $dx:ident, $n:expr) => (
        #[allow(unused_variables)]
        for ($x, $dx) in BoundIter::new($lo, $hi, $n) {
            $total += $exp * $dx;
        }
    );

    (@loop $total:ident; $integrand:expr; $lo:expr, $hi:expr, $x:ident, $dx:ident, $n:expr; $($tail:tt)*) => (
        for ($x, $dx) in BoundIter::new($lo, $hi, $n) {
            int!(@loop $total; $integrand * $dx; $($tail)*);
        }
    );

    ($integrand:expr; $($lo:expr, $hi:expr, $x:ident, $dx:ident $(, $n:expr)?);+) => (
        {
            let mut total = 0.0;
            int!(@loop total; $integrand; $($lo, $hi, $x, $dx, int!(@n $($n)?));+);
            total
        }
    );
}
