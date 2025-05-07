#![feature(test)]
extern crate test;

include!("../comm/pollard_rho.rs");

#[cfg(test)]
mod tests {
    use pollard_rho::*;
    use test::{bench, black_box};
    use std::io::Write;
    fn bench<T, F: FnMut() -> T>(name: &str, mut f: F) {
        let summary = ::tests::bench::iter(&mut f);
        writeln!(&mut std::io::stderr(), "{}: {:?} +/- {:?} [{:?}, {:?}]", name, summary.mean, summary.std_dev, summary.min, summary.max).unwrap();
    }
    #[test]
    fn bench_body() {
        bench("bench_factorize_around_2_pow_32", || {
            let mut sum = 0;
            for i in 1 << 32 .. (1 << 32) + 10 {
                for (p, e) in factorize(i) {
                    sum ^= p;
                    sum ^= e as i64;
                }
            }
            sum
        });
        bench("bench_is_prime_around_2_pow_32", || {
            let mut sum = 0;
            for i in 1 << 32 .. (1 << 32) + 100 {
                if is_prime(i) {
                    sum ^= i;
                }
            }
            sum
        });
        bench("bench_is_prime_2_pow_32_plus_15", || {
            let value = black_box((1 << 32) + 15);
            is_prime(value)
        });
    }
}
