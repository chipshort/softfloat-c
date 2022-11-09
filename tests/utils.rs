#[macro_export]
macro_rules! compare_ops {
    ($($op: ident ($($input: expr),*)),*) => {true $(&& {
        // calculate using original c library
        let c_result = unsafe { softfloat_sys::$op($(ConvertInputs::convert_c($input)),*) };
        // calculate using rust port
        let r_result = unsafe { softfloat_c::$op($(ConvertInputs::convert_r($input)),*) };

        format!("{:?}", c_result) == format!("{:?}", r_result)
    })*};
}

/// Converts inputs for the C and Rust operations to the correct type
pub trait ConvertInputs: Copy + Sized {
    type COut;
    type ROut;

    fn convert(self) -> (Self::COut, Self::ROut) {
        let c = self.convert_c();
        let r = self.convert_r();

        (c, r)
    }

    fn convert_c(self) -> Self::COut;
    fn convert_r(self) -> Self::ROut;
}

impl ConvertInputs for f32 {
    type COut = softfloat_sys::float32_t;
    type ROut = softfloat_c::float32_t;

    fn convert_c(self) -> Self::COut {
        Self::COut { v: self.to_bits() }
    }
    fn convert_r(self) -> Self::ROut {
        Self::ROut { v: self.to_bits() }
    }
}

impl ConvertInputs for f64 {
    type COut = softfloat_sys::float64_t;
    type ROut = softfloat_c::float64_t;

    fn convert_c(self) -> Self::COut {
        Self::COut { v: self.to_bits() }
    }
    fn convert_r(self) -> Self::ROut {
        Self::ROut { v: self.to_bits() }
    }
}

/// Inputs that need to be converted for the C and Rust version, but to the same type for both
pub trait SameConvertInput: Copy + Sized {
    type Out;
    fn convert(self) -> Self::Out;
}

impl<S: SameConvertInput> ConvertInputs for S {
    type COut = S::Out;
    type ROut = S::Out;

    fn convert_c(self) -> Self::COut {
        self.convert()
    }
    fn convert_r(self) -> Self::ROut {
        self.convert()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RoundingMode(u8);

impl quickcheck::Arbitrary for RoundingMode {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        RoundingMode(*g.choose(&[0u8, 1, 2, 3, 4, 6]).unwrap())
    }
}

impl SameConvertInput for RoundingMode {
    type Out = u8;
    fn convert(self) -> Self::Out {
        self.0
    }
}

/// Inputs that can stay the same for both the C and Rust versions
pub trait NoopConvertInput: Copy + Sized {}

impl NoopConvertInput for bool {}

impl<N: NoopConvertInput> SameConvertInput for N {
    type Out = N;

    fn convert(self) -> Self::Out {
        self
    }
}
