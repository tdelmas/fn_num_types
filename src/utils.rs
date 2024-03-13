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
    /// assert_eq!(Possible::Yes, Possible::Yes | Possible::Should);
    /// assert_eq!(Possible::Yes, Possible::Should | Possible::Yes);
    /// assert_eq!(Possible::Should, Possible::Should | Possible::ShouldNot);
    /// assert_eq!(Possible::Should, Possible::ShouldNot | Possible::Should);
    /// assert_eq!(Possible::ShouldNot, Possible::ShouldNot | Possible::No);
    /// assert_eq!(Possible::ShouldNot, Possible::No | Possible::ShouldNot);
    /// assert_eq!(Possible::No, Possible::No | Possible::No);
    /// ```
    pub fn any(a: Self, b: Self) -> Self {
        std::cmp::max(a, b)
    }
}

impl core::ops::BitOr for Possible {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        std::cmp::max(self, rhs)
    }
}

impl core::ops::BitAnd for Possible {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        std::cmp::min(self, rhs)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FloatPossibilities {
    pub nan: Possible,
    pub zero: Possible,
    pub infinite: Possible,
    pub positive: Possible,
    pub negative: Possible,
}

pub type FP = FloatPossibilities;

impl FloatPossibilities {
    /// Returns true if the value is accepted
    ///
    /// ```
    /// use fn_num_types::{FloatPossibilities, Possible};
    ///
    /// let possibilities = FloatPossibilities {
    ///     nan: Possible::Yes,
    ///     zero: Possible::Yes,
    ///     infinite: Possible::Yes,
    ///     positive: Possible::Yes,
    ///     negative: Possible::Yes,
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

        if value.is_sign_positive() && self.positive == Possible::No {
            return false;
        }

        if value.is_sign_negative() && self.negative == Possible::No {
            return false;
        }

        true
    }

    pub fn union(&self, rhs: &Self) -> Self {
        FP {
            nan: self.nan | rhs.nan,
            zero: self.zero | rhs.zero,
            infinite: self.infinite | rhs.infinite,
            positive: self.positive | rhs.positive,
            negative: self.negative | rhs.negative,
        }
    }
}

pub const ZERO_POSSIBILITIES: FP = FP {
    nan: Possible::No,
    zero: Possible::Yes,
    infinite: Possible::No,
    positive: Possible::Yes,
    negative: Possible::No,
};

pub const ZERO_NEG_POSSIBILITIES: FP = FP {
    nan: Possible::No,
    zero: Possible::Yes,
    infinite: Possible::No,
    positive: Possible::No,
    negative: Possible::Yes,
};

pub const INF_POSSIBILITIES: FP = FP {
    nan: Possible::No,
    zero: Possible::No,
    infinite: Possible::Yes,
    positive: Possible::Yes,
    negative: Possible::Yes,
};

pub const INF_NEG_POSSIBILITIES: FP = FP {
    nan: Possible::No,
    zero: Possible::No,
    infinite: Possible::Yes,
    positive: Possible::No,
    negative: Possible::Yes,
};

#[derive(Clone, Copy, Debug)]
pub enum FnArgFloat {
    F32(FloatPossibilities),
    F64(FloatPossibilities),
}

pub(crate) fn return_fp<F>(lhs: &FnArgFloat, possibilities: F) -> FnArgFloat
where
    F: FnOnce(&FP) -> FP,
{
    match lhs {
        FnArgFloat::F32(fp) => FnArgFloat::F32(possibilities(fp)),
        FnArgFloat::F64(fp) => FnArgFloat::F64(possibilities(fp)),
    }
}

pub(crate) fn return_fp2<F>(lhs: &FnArgFloat, rhs: &FnArgFloat, possibilities: F) -> FnArgFloat
where
    F: FnOnce(&FP, &FP) -> FP,
{
    match (lhs, rhs) {
        (FnArgFloat::F32(fp1), FnArgFloat::F32(fp2)) => FnArgFloat::F32(possibilities(fp1, fp2)),
        (FnArgFloat::F64(fp1), FnArgFloat::F64(fp2)) => FnArgFloat::F64(possibilities(fp1, fp2)),
        _ => panic!("Different types"),
    }
}
