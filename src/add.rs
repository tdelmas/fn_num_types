use crate::{return_fp2, FnArgFloat, Possible, FP};

pub fn add(a: &FnArgFloat, b: &FnArgFloat) -> FnArgFloat {
    return_fp2(a, b, |fp1, fp2| {
        let overflow = FP {
            nan: Possible::No,
            zero: Possible::No,
            infinite: Possible::ShouldNot,
            positive: Possible::No,
            negative: Possible::No,
        };

        let mut res = fp1.union(fp2);

        // Negative overflow
        if (fp1.negative & fp2.negative) != Possible::No {
            res = res.union(&overflow);
        }

        // Positive overflow
        if (fp1.positive & fp2.positive) != Possible::No {
            res = res.union(&overflow);
        }

        // Opposit infinities
        let both_inf = fp1.infinite & fp2.infinite;
        let opposite = (fp1.positive & fp2.negative) | (fp1.negative & fp2.positive);
        res.nan = res.nan | (both_inf & opposite);

        // Zero
        res.zero = res.zero | opposite;

        res
    })
}
