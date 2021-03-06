#![feature(test)]

#[cfg(feature = "comparitive")]
extern crate decimal;
extern crate rust_decimal;
extern crate test;

use rust_decimal::Decimal;
use std::str::FromStr;

macro_rules! bench_decimal_op {
    ($name:ident, $op:tt, $y:expr) => {
        #[bench]
        fn $name(b: &mut ::test::Bencher) {
            let x = Decimal::from_str("2.01").unwrap();
            let y = Decimal::from_str($y).unwrap();
            b.iter(|| {
                let result = x $op y;
                ::test::black_box(result);
            });
        }
    }
}

macro_rules! bench_fold_op {
    ($name:ident, $op:tt, $init:expr, $count:expr) => {
        #[bench]
        fn $name(b: &mut ::test::Bencher) {
            fn fold(values: &[Decimal]) -> Decimal {
                let mut acc: Decimal = $init.into();
                for value in values {
                    acc = acc $op value;
                }
                acc
            }

            let values: Vec<Decimal> = test::black_box((1..$count).map(|i| i.into()).collect());
            b.iter(|| {
                let result = fold(&values);
                ::test::black_box(result);
            });
        }
    }
}

/* Add */
bench_decimal_op!(add_one, +, "1");
bench_decimal_op!(add_two, +, "2");
bench_decimal_op!(add_one_hundred, +, "100");
bench_decimal_op!(add_point_zero_one, +, "0.01");
bench_decimal_op!(add_negative_point_five, +, "-0.5");
bench_decimal_op!(add_pi, +, "3.1415926535897932384626433832");
bench_decimal_op!(add_negative_pi, +, "-3.1415926535897932384626433832");

bench_fold_op!(sum_10k, +, 0, 10_000);

/* Sub */
bench_decimal_op!(sub_one, -, "1");
bench_decimal_op!(sub_two, -, "2");
bench_decimal_op!(sub_one_hundred, -, "100");
bench_decimal_op!(sub_point_zero_one, -, "0.01");
bench_decimal_op!(sub_negative_point_five, -, "-0.5");
bench_decimal_op!(sub_pi, -, "3.1415926535897932384626433832");
bench_decimal_op!(sub_negative_pi, -, "-3.1415926535897932384626433832");

bench_fold_op!(dec_10k, -, 5_000_000, 10_000);

/* Mul */
bench_decimal_op!(mul_one, *, "1");
bench_decimal_op!(mul_two, *, "2");
bench_decimal_op!(mul_one_hundred, *, "100");
bench_decimal_op!(mul_point_zero_one, *, "0.01");
bench_decimal_op!(mul_negative_point_five, *, "-0.5");
bench_decimal_op!(mul_pi, *, "3.1415926535897932384626433832");
bench_decimal_op!(mul_negative_pi, *, "-3.1415926535897932384626433832");

/* Div */
bench_decimal_op!(div_one, /, "1");
bench_decimal_op!(div_two, /, "2");
bench_decimal_op!(div_one_hundred, /, "100");
bench_decimal_op!(div_point_zero_one, /, "0.01");
bench_decimal_op!(div_negative_point_five, /, "-0.5");
bench_decimal_op!(div_pi, /, "3.1415926535897932384626433832");
bench_decimal_op!(div_negative_pi, /, "-3.1415926535897932384626433832");

bench_fold_op!(frac_10k, /, Decimal::max_value(), 10_000);

#[cfg(feature = "comparitive")]
mod comparitive {
    use decimal::d128;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    macro_rules! bench_dec_op {
        ($name:ident, $op:tt) => {
            #[bench]
            fn $name(b: &mut ::test::Bencher) {
                let x = Decimal::from_str("2.01").unwrap();
                let y = Decimal::from_str("3.1415926535897932384626433832").unwrap();
                b.iter(|| {
                    let result = x $op y;
                    ::test::black_box(result);
                });
            }
        }
    }

    macro_rules! bench_d128_op {
        ($name:ident, $op:tt) => {
            #[bench]
            fn $name(b: &mut ::test::Bencher) {
                let x = d128::from_str("2.01").unwrap();
                let y = d128::from_str("3.1415926535897932384626433832").unwrap();
                b.iter(|| {
                    let result = x $op y;
                    ::test::black_box(result);
                });
            }
        }
    }

    bench_dec_op!(add_dec, +);
    bench_d128_op!(add_d128, +);
    bench_dec_op!(sub_dec, -);
    bench_d128_op!(sub_d128, -);
    bench_dec_op!(mul_dec, *);
    bench_d128_op!(mul_d128, *);
    bench_dec_op!(div_dec, /);
    bench_d128_op!(div_d128, /);
}
#[cfg(not(feature = "comparitive"))]
mod comparitive {

}
