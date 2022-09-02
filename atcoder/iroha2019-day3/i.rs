// Finds all primes <= lim.
// Verified: https://atcoder.jp/contests/abc195/submissions/22273725
fn primes(lim: usize) -> Vec<bool> {
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
    pr
}

// Max found: 9200000494408433 9200000494410451 (94 primes)

fn main() {
    let pr = primes(100_000_000);
    let st = 9_200_000_000_000_000i64;
    let len = 500_000_000;
    let mut dp = vec![true; len];
    for p in 2..pr.len() {
        let pp = p as i64;
        if pr[p] {
            let l = (st + pp - 1) / pp;
            let r = (st + len as i64 + pp - 1) / pp;
            for i in l..r {
                dp[(i * pp - st) as usize] = false;
            }
        }
    }
    let mut acc = vec![0; len + 1];
    for i in 0..len {
        acc[i + 1] = acc[i] + if dp[i] { 1 } else { 0 };
    }
    let mut ma = (0, 0);
    for i in 0..=len - 2019 {
        ma = std::cmp::max(ma, (acc[i + 2019] - acc[i], i as i64 + st));
    }
    eprintln!("#primes = {}", ma.0);
    println!("{} {}", ma.1, ma.1 + 2018);
}
