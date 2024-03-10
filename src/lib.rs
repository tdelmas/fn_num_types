#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Range {
    #[default]
    Full,
    Negative,
    Positive,
}

/// Is that value possible to reach?
///
/// E.g.: If `x` is a floating point strictly positive finite number:
/// - `x + 1.0 negative ?` is `Possible::No`
/// - `x + 1.0 positive ?` is `Possible::Yes`
/// - `x * x == 0.0 ?` is `Possible::ShouldNot` because `f64::MIN_POSITIVE * f64::MIN_POSITIVE == 0.0`
/// - `sin(x) == 0.0 ?` is `Possible::Should` because mathematically it should be possible, but because of the rounding error, it may not happen
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd)]
pub enum Possible {
    No,
    // Theorically no, but may happen because of the rounding error
    ShouldNot,
    // Theorically yes, but may not happen because of the rounding error
    Should,
    #[default]
    Yes,
}

impl Possible {
    /// If something is possible for two reasons,
    /// we return the strongest one :
    ///
    /// ```
    /// use fn_num_types::Possible;
    ///
    /// assert_eq!(Possible::Yes, Possible::any(Possible::Yes, Possible::Should));
    /// assert_eq!(Possible::Yes, Possible::any(Possible::Should, Possible::Yes));
    /// assert_eq!(Possible::Should, Possible::any(Possible::Should, Possible::ShouldNot));
    /// assert_eq!(Possible::Should, Possible::any(Possible::ShouldNot, Possible::Should));
    /// assert_eq!(Possible::ShouldNot, Possible::any(Possible::ShouldNot, Possible::No));
    /// assert_eq!(Possible::ShouldNot, Possible::any(Possible::No, Possible::ShouldNot));
    /// assert_eq!(Possible::No, Possible::any(Possible::No, Possible::No));
    /// ```
    pub fn any(a: Self, b: Self) -> Self {
        std::cmp::max(a, b)
    }
}

impl Range {
    pub fn opposite(&self) -> Self {
        match self {
            Range::Full => Range::Full,
            Range::Negative => Range::Positive,
            Range::Positive => Range::Negative,
        }
    }

    pub fn can_be_positive(&self) -> Possible {
        match self {
            Range::Full | Range::Positive => Possible::Yes,
            Range::Negative => Possible::No,
        }
    }

    pub fn can_be_negative(&self) -> Possible {
        match self {
            Range::Full | Range::Negative => Possible::Yes,
            Range::Positive => Possible::No,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FloatPossibilities {
    pub nan: Possible,
    pub zero: Possible,
    pub infinite: Possible,
    pub range: Range,
}

type FP = FloatPossibilities;

impl FloatPossibilities {
    /// Returns true if the value is accepted
    ///
    /// ```
    /// use fn_num_types::{FloatPossibilities, Possible, Range};
    ///
    /// let possibilities = FloatPossibilities {
    ///     nan: Possible::Yes,
    ///     zero: Possible::Yes,
    ///     infinite: Possible::Yes,
    ///     range: Range::Full,
    /// };
    ///
    /// assert!(possibilities.accept(f64::NAN));
    /// assert!(possibilities.accept(f64::INFINITY));
    /// assert!(possibilities.accept(f64::NEG_INFINITY));
    /// assert!(possibilities.accept(0.0));
    /// assert!(possibilities.accept(-0.0));
    /// assert!(possibilities.accept(1.0));
    /// assert!(possibilities.accept(-1.0));
    /// ```
    pub fn accept(&self, value: f64) -> bool {
        if value.is_nan() {
            return self.nan != Possible::No;
        }

        if value.is_infinite() && self.infinite == Possible::No {
            return false;
        }

        if value == 0.0 && self.zero == Possible::No {
            return false;
        }

        if value.is_sign_positive() && self.range.can_be_positive() == Possible::No {
            return false;
        }

        if value.is_sign_negative() && self.range.can_be_negative() == Possible::No {
            return false;
        }

        true
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FnArgFloat {
    F32(FloatPossibilities),
    F64(FloatPossibilities),
}

fn return_fp<F>(lhs: &FnArgFloat, possibilities: F) -> FnArgFloat
where
    F: FnOnce(&FP) -> FP,
{
    match lhs {
        FnArgFloat::F32(fp) => FnArgFloat::F32(possibilities(fp)),
        FnArgFloat::F64(fp) => FnArgFloat::F64(possibilities(fp)),
    }
}

pub mod core {
    pub mod ops {
        use crate::*;

        pub fn neg(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                range: fp.range.opposite(),
                ..*fp
            })
        }

        pub fn abs(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                range: Range::Positive,
                ..*fp
            })
        }

        pub fn ceil(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                zero: Possible::any(fp.zero, fp.range.can_be_negative()),
                ..*fp
            })
        }

        pub fn floor(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                zero: Possible::any(fp.zero, fp.range.can_be_positive()),
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
                nan: Possible::any(fp.nan, fp.infinite),
                // Returns POSITIVE zero if the factional part is zero
                range: if fp.range.can_be_negative() == Possible::Yes {
                    Range::Full
                } else {
                    Range::Positive
                },
                infinite: fp.infinite,
            })
        }

        pub fn signum(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                zero: Possible::No,
                infinite: Possible::No,
                range: fp.range,
                nan: fp.nan,
            })
        }

        pub fn sqrt(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                nan: Possible::any(fp.nan, fp.range.can_be_negative()),
                ..*fp
            })
        }

        pub fn exp(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                range: Range::Positive,
                zero: fp.range.can_be_negative(),
                infinite: fp.range.can_be_positive(),
                nan: fp.nan,
            })
        }

        pub fn exp2(lhs: &FnArgFloat) -> FnArgFloat {
            exp(lhs)
        }

        pub fn ln(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                range: Range::Full,
                zero: fp.range.can_be_positive(),
                infinite: Possible::any(fp.infinite, fp.zero),
                nan: Possible::any(fp.nan, fp.range.can_be_negative()),
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
                range: Range::Full,
                zero: Possible::Yes,
                infinite: Possible::No,
                nan: Possible::any(fp.nan, fp.infinite),
            })
        }

        pub fn cos(lhs: &FnArgFloat) -> FnArgFloat {
            sin(lhs)
        }

        pub fn tan(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                range: Range::Full,
                zero: Possible::Yes,
                infinite: Possible::Yes,
                nan: Possible::any(fp.nan, fp.infinite),
            })
        }

        pub fn asin(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                range: fp.range,
                zero: fp.zero,
                infinite: Possible::No,
                nan: Possible::Yes,
            })
        }

        pub fn acos(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                range: Range::Positive,
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
                infinite: fp.range.can_be_positive(),
                ..*fp
            })
        }

        pub fn ln_1p(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                nan: Possible::any(fp.nan, fp.range.can_be_negative()),
                infinite: Possible::any(fp.infinite, fp.range.can_be_negative()),
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
                range: Range::Positive,
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
            return_fp(lhs, |fp| FP {
                range: Range::Positive,
                zero: Possible::Yes,
                infinite: Possible::Yes,
                nan: Possible::Yes,
            })
        }

        pub fn atanh(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                range: fp.range,
                zero: fp.zero,
                infinite: Possible::Yes,
                nan: Possible::Yes,
            })
        }

        pub fn recip(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                range: fp.range,
                zero: fp.infinite,
                infinite: fp.zero,
                nan: fp.nan,
            })
        }

        // TODO: add argument
        pub fn powi(lhs: &FnArgFloat) -> FnArgFloat {
            return_fp(lhs, |fp| FP {
                range: if fp.range.can_be_negative() == Possible::Yes {
                    Range::Full
                } else {
                    Range::Positive
                },
                zero: Possible::Yes,
                infinite: Possible::Yes,
                nan: fp.nan,
            })
        }
    }
}
