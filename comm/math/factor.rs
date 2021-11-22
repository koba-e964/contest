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
