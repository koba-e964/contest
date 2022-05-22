// Finds all primes <= lim.
// Verified: https://atcoder.jp/contests/abc195/submissions/22273725
fn primes(lim: usize) -> Vec<usize> {
    if lim <= 1 {
        return vec![];
    }
    let mut pr = vec![true; lim + 1];
    pr[0] = false;
    pr[1] = false;
    for i in 2..=lim {
        if !pr[i] {
            continue;
        }
        for j in 2..=lim / i {
            pr[i * j] = false;
        }
    }
    (2..=lim).filter(|&i| pr[i]).collect()
}
