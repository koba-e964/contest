type Int = i32;
fn ex_gcd(x: Int, y: Int) -> (Int, Int, Int) {
    if y == 0 {
        return (x, 1, 0);
    }
    let q = x / y;
    let r = x % y;
    let (res, a, b) = ex_gcd(y, r);
    let tmp = a - q * b;
    (res, b, tmp)
}

// Calculates the intersection of two arithmetic progressions,
// x[n] = a + b * n and y[n] = c + d * n (n >= 0).
// p1 = (a, b), p2 = (c, d)
// Verified by: https://atcoder.jp/contests/ukuku09/submissions/34522507
fn arith_prog_intersect(p1: (Int, Int), p2: (Int, Int)) -> Option<(Int, Int)> {
    if p1.0 > p2.0 {
        return arith_prog_intersect(p2, p1);
    }
    let (g, _u, v) = ex_gcd(p1.1, p2.1);
    let lcm = p1.1 / g * p2.1;
    if (p1.0 - p2.0) % g != 0 {
        return None;
    }
    let mut diff = (p2.0 - p1.0) / g;
    diff *= -v % (p1.1 / g);
    diff %= p1.1 / g;
    if diff < 0 {
        diff += p1.1 / g;
    }
    let x = p2.0 + diff * p2.1;
    Some((x, lcm))
}
