//! Compares this crate with the C implementation (in the form of the [`softfloat-sys`] crate).
//! Currently only f32 and f64 operations are tested.

use quickcheck::quickcheck;
use utils::*;

mod utils;

quickcheck! {
    fn f32_binops(a: f32, b: f32) -> bool {
        compare_ops!(
            f32_add(a, b),
            f32_sub(a, b),
            f32_mul(a, b),
            f32_div(a, b),
            f32_rem(a, b),
            f32_sqrt(a),
            f32_eq(a, b),
            f32_le(a, b),
            f32_lt(a, b),
            f32_eq_signaling(a, b),
            f32_le_quiet(a, b),
            f32_lt_quiet(a, b)
        )
    }

    fn f32_unops(a: f32) -> bool {
        compare_ops!(
            f32_to_f16(a),
            f32_to_f64(a),
            f32_to_extF80(a),
            f32_to_f128(a),
            f32_isSignalingNaN(a)
        )
    }

    fn f32_round_ops(a: f32, rounding_mode: RoundingMode, exact: bool) -> bool {
        compare_ops!(
            f32_to_ui32(a, rounding_mode, exact),
            f32_to_ui64(a, rounding_mode, exact),
            f32_to_i32(a, rounding_mode, exact),
            f32_to_i64(a, rounding_mode, exact),
            f32_roundToInt(a, rounding_mode, exact)
        )
    }

    fn f32_exact_ops(a: f32, exact: bool) -> bool {
        compare_ops!(
            f32_to_ui32_r_minMag(a, exact),
            f32_to_ui64_r_minMag(a, exact),
            f32_to_i32_r_minMag(a, exact),
            f32_to_i64_r_minMag(a, exact)
        )
    }

    fn f64_binops(a: f64, b: f64) -> bool {
        compare_ops!(
            f64_add(a, b),
            f64_sub(a, b),
            f64_mul(a, b),
            f64_div(a, b),
            f64_rem(a, b),
            f64_sqrt(a),
            f64_eq(a, b),
            f64_le(a, b),
            f64_lt(a, b),
            f64_eq_signaling(a, b),
            f64_le_quiet(a, b),
            f64_lt_quiet(a, b)
        )
    }

    fn f64_unops(a: f64) -> bool {
        compare_ops!(
            f64_to_f16(a),
            f64_to_f32(a),
            f64_to_extF80(a),
            f64_to_f128(a),
            f64_isSignalingNaN(a)
        )
    }

    fn f64_round_ops(a: f64, rounding_mode: RoundingMode, exact: bool) -> bool {
        compare_ops!(
            f64_to_ui32(a, rounding_mode, exact),
            f64_to_ui64(a, rounding_mode, exact),
            f64_to_i32(a, rounding_mode, exact),
            f64_to_i64(a, rounding_mode, exact),
            f64_roundToInt(a, rounding_mode, exact)
        )
    }

    fn f64_exact_ops(a: f64, exact: bool) -> bool {
        compare_ops!(
            f64_to_ui32_r_minMag(a, exact),
            f64_to_ui64_r_minMag(a, exact),
            f64_to_i32_r_minMag(a, exact),
            f64_to_i64_r_minMag(a, exact)
        )
    }
}
