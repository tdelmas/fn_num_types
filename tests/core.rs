use fn_num_types::{FloatPossibilities, FnArgFloat, Possible};

macro_rules! get_test_values {
    ($float_type:ident) => {
        [
            core::$float_type::NAN,
            core::$float_type::NEG_INFINITY,
            core::$float_type::MIN,
            core::$float_type::MIN / 2.0,
            -core::$float_type::consts::PI,
            -core::$float_type::consts::E,
            -2.0,
            -core::$float_type::consts::FRAC_PI_2,
            -1.0,
            -core::$float_type::MIN_POSITIVE,
            -1.0e-308, // Smallest negative subnormal. Rounded to zero for f32
            -0.0,
            0.0,
            1.0e-308, // Smallest positive subnormal. Rounded to zero for f32
            core::$float_type::MIN_POSITIVE,
            1.0,
            core::$float_type::consts::FRAC_PI_2,
            2.0,
            core::$float_type::consts::E,
            core::$float_type::consts::PI,
            core::$float_type::MAX / 2.0,
            core::$float_type::MAX,
            core::$float_type::INFINITY,
        ]
    };
}

const YESNO: [Possible; 2] = [Possible::Yes, Possible::No];

fn get_possibilities() -> Vec<FloatPossibilities> {
    let mut possibles = vec![];

    for nan in YESNO {
        for zero in YESNO {
            for infinite in YESNO {
                for positive in YESNO {
                    for negative in YESNO {
                        possibles.push(FloatPossibilities {
                            nan,
                            zero,
                            infinite,
                            positive,
                            negative,
                        });
                    }
                }
            }
        }
    }

    possibles
}

#[test]
fn test_values() {
    let values = get_test_values!(f64);

    for w in values.windows(2) {
        let a = w[0];
        let b = w[1];

        if a.is_nan() || b.is_nan() {
            continue;
        }

        if a == b {
            assert_eq!(a, 0.0);
            assert_eq!(b, 0.0);
            assert!(a.is_sign_negative());
            assert!(b.is_sign_positive());
        } else {
            assert!(a < b);
        }
    }
}

macro_rules! generate_tests {
    ($float:ident, $mod:ident) => {
        fn test_op(name: &str, op: fn($float) -> $float, ty: fn(&FnArgFloat) -> FnArgFloat) {
            let possibles = get_possibilities();
            let values = get_test_values!($float);

            for v in values.iter() {
                for p in possibles.iter() {
                    if !p.accept(*v) {
                        continue;
                    }

                    let result = op(*v);
                    let res_p = ty(&FnArgFloat::$mod(*p));

                    println!("Testing {name}");
                    println!("Testing {v:?} = {result:?}");
                    println!("Testing {p:?} = {res_p:?}");

                    match res_p {
                        FnArgFloat::$mod(res_p) => {
                            assert!(res_p.accept(result));
                        }
                        _ => panic!("Invalid result"),
                    }
                }
            }
        }

        fn test_op2(
            name: &str,
            op: fn($float, $float) -> $float,
            ty: fn(&FnArgFloat, &FnArgFloat) -> FnArgFloat,
        ) {
            let possibles = get_possibilities();
            let values = get_test_values!($float);

            for v1 in values.iter() {
                for p1 in possibles.iter() {
                    if !p1.accept(*v1) {
                        continue;
                    }
                    for v2 in values.iter() {
                        for p2 in possibles.iter() {
                            if !p2.accept(*v2) {
                                continue;
                            }

                            let result = op(*v1, *v2);
                            let res_p = ty(&FnArgFloat::$mod(*p1), &FnArgFloat::$mod(*p2));

                            println!("Testing {name}");
                            println!("Testing {v1:?} {v2:?} = {result:?}");
                            println!("Testing {p1:?} {p2:?} = {res_p:?}");

                            match res_p {
                                FnArgFloat::$mod(res_p) => {
                                    assert!(res_p.accept(result));
                                }
                                _ => panic!("Invalid result"),
                            }
                        }
                    }
                }
            }
        }

        macro_rules! test_op {
            ($op:ident) => {
                test_op(stringify!($op), |x| x.$op(), fn_num_types::core::ops::$op);
            };
        }

        #[test]
        fn test_ops() {
            test_op("neg", |x| -x, fn_num_types::core::ops::neg);
            test_op!(abs);
            test_op!(ceil);
            test_op!(floor);
            test_op!(round);
            test_op!(trunc);
            test_op!(fract);
            test_op!(signum);
            test_op!(sqrt);
            test_op!(exp);
            test_op!(exp2);
            test_op!(ln);
            test_op!(log2);
            test_op!(log10);
            test_op!(to_degrees);
            test_op!(to_radians);
            test_op!(cbrt);
            test_op!(sin);
            test_op!(cos);
            test_op!(tan);
            test_op!(asin);
            test_op!(acos);
            test_op!(atan);
            test_op!(exp_m1);
            test_op!(ln_1p);
            test_op!(sinh);
            test_op!(cosh);
            test_op!(tanh);
            test_op!(asinh);
            test_op!(acosh);
            test_op!(atanh);
            test_op!(recip);
            test_op("powi", |x| x.powi(2), |x| fn_num_types::core::ops::powi(x));
        }

        #[test]
        fn test_ops2() {
            test_op2(
                "add",
                |x, y| x + y,
                |x, y| fn_num_types::core::ops::add(x, y),
            );
        }
    };
}

// mod f32 {
//     use super::*;
//     generate_tests!(f32, F32);
// }

mod f64 {
    use super::*;
    generate_tests!(f64, F64);
}
