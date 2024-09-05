include!("pollard_rho.rs");

fn main() {
    use pollard_rho::*;
    assert_eq!(factorize(4681), vec![(31, 1), (151, 1)]);
    // https://lpha-z.hatenablog.com/entry/2023/01/15/231500
    assert_eq!(factorize(124376107291), vec![(352523, 1), (352817, 1)]);
    assert_eq!(factorize(273772559), vec![(15881, 1), (17239, 1)]);
    assert_eq!(factorize(2059), vec![(29, 1), (71, 1)]);
    assert_eq!(factorize(385515865499), vec![(599477, 1), (643087, 1)]);

    // Performs Eratosthenes sieve
    const W: usize = 1_000_000;
    let mut pr = vec![true; W];
    pr[0] = false;
    pr[1] = false;
    for i in 2..W {
        if !pr[i] { continue; }
        for j in 2..(W - 1) / i + 1 {
            pr[i * j] = false;
        }
    }
    for i in 1..W {
        assert_eq!(is_prime(i as i64), pr[i], "i = {}", i);
    }
}
