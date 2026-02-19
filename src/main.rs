/// Fast inverse square root for f64.
/// Returns an approximation of 1 / sqrt(x).
pub fn fast_inv_sqrt_f64(x: f64) -> f64 {
    if x <= 0.0 || !x.is_finite() {
        return f64::NAN;
    }

    let xhalf = 0.5 * x;

    // Bit-level approximation
    let mut i = x.to_bits();
    i = 0x5fe6eb50c7b537a9u64 - (i >> 1);
    let mut y = f64::from_bits(i);

    // One Newton-Raphson iteration
    y = y * (1.5 - xhalf * y * y);

    y
}

fn main() {
    let x = 10.0f64;

    let approx = fast_inv_sqrt_f64(x);
    let exact = 1.0 / x.sqrt();

    println!("x       = {}", x);
    println!("approx  = {}", approx);
    println!("exact   = {}", exact);
    println!("error   = {}", (approx - exact).abs());
}