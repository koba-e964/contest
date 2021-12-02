fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

fn factorize(mut x: i64) -> Vec<(i64, usize)> {
    let mut p = 2;
    let mut ans = vec![];
    while p * p <= x {
        let mut e = 0;
        while x % p == 0 {
            x /= p;
            e += 1;
        }
        if e > 0 {
            ans.push((p, e));
        }
        p += 1;
    }
    if x > 1 {
        ans.push((x, 1));
    }
    ans
}

// Returns a table pr that satisfies pr[i] <=> i is prime (0 <= i < n).
// Complexity: O(n log log n)
fn is_primes_tbl(n: usize) -> Vec<bool> {
    if n <= 2 {
        return vec![false; n];
    }
    let mut pr = vec![true; n];
    pr[0] = false;
    pr[1] = false;
    for i in 2..n {
        if !pr[i] { continue; }
        for j in 2..(n - 1) / i {
            pr[i * j] = false;
        }
    }
    pr
}
