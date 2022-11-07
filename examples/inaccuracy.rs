use softfloat::{f64_mul, float64_t};

fn main() {
    let x = float64_t::from_bits(1.1f64.to_bits());
    let y = float64_t::from_bits(2.2f64.to_bits());

    assert_eq!(
        2.4200000000000004f64.to_bits(),
        unsafe { f64_mul(x, y) }.to_bits(),
        "1.1 times 2.2 has a small rounding error"
    );
}
