mod add;
mod utils;

pub use utils::*;

pub mod core {
    pub mod ops {

        use crate::*;

        pub use add::add;

        pub fn neg(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                positive: fp.negative,
                negative: fp.positive,
                ..*fp
            })
        }

        pub fn abs(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                positive: fp.positive | fp.negative,
                negative: Possible::No,
                ..*fp
            })
        }

        pub fn ceil(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                zero: fp.zero | fp.negative,
                ..*fp
            })
        }

        pub fn floor(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                zero: fp.zero | fp.positive,
                ..*fp
            })
        }

        pub fn round(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                zero: Possible::Yes,
                ..*fp
            })
        }

        pub fn trunc(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                zero: Possible::Yes,
                ..*fp
            })
        }

        pub fn fract(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                zero: Possible::Yes,
                nan: fp.nan | fp.infinite,
                // Return POSITIVE zero if the factional part is zero
                positive: fp.positive | fp.negative,
                negative: fp.negative,
                infinite: fp.infinite,
            })
        }

        pub fn signum(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                zero: Possible::No,
                infinite: Possible::No,
                ..*fp
            })
        }

        pub fn sqrt(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                nan: fp.nan | fp.negative,
                ..*fp
            })
        }

        pub fn exp(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                positive: Possible::Yes,
                negative: Possible::No,
                zero: fp.negative,
                infinite: fp.positive,
                nan: fp.nan,
            })
        }

        pub fn exp2(lhs: &FnArgFloat) -> FnArgFloat {
            exp(lhs)
        }

        pub fn ln(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                positive: Possible::Yes,
                negative: Possible::Yes,
                zero: fp.positive,
                infinite: fp.infinite | fp.zero,
                nan: fp.nan | fp.negative,
            })
        }

        pub fn log2(lhs: &FnArgFloat) -> FnArgFloat {
            ln(lhs)
        }

        pub fn log10(lhs: &FnArgFloat) -> FnArgFloat {
            ln(lhs)
        }

        pub fn to_degrees(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                // May reach Infinity with large values
                infinite: Possible::Yes,
                ..*fp
            })
        }

        pub fn to_radians(lhs: &FnArgFloat) -> FnArgFloat {
            *lhs
        }

        pub fn cbrt(lhs: &FnArgFloat) -> FnArgFloat {
            *lhs
        }

        pub fn sin(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                positive: Possible::Yes,
                negative: Possible::Yes,
                zero: Possible::Yes,
                infinite: Possible::No,
                nan: fp.nan | fp.infinite,
            })
        }

        pub fn cos(lhs: &FnArgFloat) -> FnArgFloat {
            sin(lhs)
        }

        pub fn tan(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                positive: Possible::Yes,
                negative: Possible::Yes,
                zero: Possible::Yes,
                infinite: Possible::Yes,
                nan: fp.nan | fp.infinite,
            })
        }

        pub fn asin(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                zero: fp.zero,
                infinite: Possible::No,
                nan: Possible::Yes,
                ..*fp
            })
        }

        pub fn acos(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |_| FP {
                positive: Possible::Yes,
                negative: Possible::No,
                zero: Possible::Yes,
                infinite: Possible::No,
                nan: Possible::Yes,
            })
        }

        pub fn atan(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                infinite: Possible::No,
                ..*fp
            })
        }

        pub fn exp_m1(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                infinite: fp.positive,
                ..*fp
            })
        }

        pub fn ln_1p(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                nan: fp.nan | fp.negative,
                infinite: fp.infinite | fp.negative,
                ..*fp
            })
        }

        pub fn sinh(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                infinite: Possible::Yes,
                ..*fp
            })
        }

        pub fn cosh(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                positive: Possible::Yes,
                negative: Possible::No,
                zero: Possible::No,
                infinite: Possible::Yes,
                nan: fp.nan,
            })
        }

        pub fn tanh(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                infinite: Possible::No,
                ..*fp
            })
        }

        pub fn asinh(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                infinite: Possible::Yes,
                ..*fp
            })
        }

        pub fn acosh(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |_| FP {
                positive: Possible::Yes,
                negative: Possible::No,
                zero: Possible::Yes,
                infinite: Possible::Yes,
                nan: Possible::Yes,
            })
        }

        pub fn atanh(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                infinite: Possible::Yes,
                nan: Possible::Yes,
                ..*fp
            })
        }

        pub fn recip(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                zero: fp.infinite,
                infinite: fp.zero,
                ..*fp
            })
        }

        pub fn powi(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                positive: Possible::Yes,
                zero: Possible::Yes,
                infinite: Possible::Yes,
                ..*fp
            })
        }
    }
}
