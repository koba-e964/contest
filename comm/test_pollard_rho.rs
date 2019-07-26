include!("pollard_rho.rs");

fn main() {
    use pollard_rho::*;
    assert_eq!(factorize(4681), vec![(31, 1), (151, 1)]);

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
