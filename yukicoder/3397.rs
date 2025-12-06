fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn quot(a: i64, b: i64) -> i64 {
    assert!(b > 0);
    let mut q = a / b;
    let r = a - q * b;
    if r < 0 {
        q -= 1;
    }
    q
}

fn calc_inner(n: i64, m: i64, a: i64, b: i64, c: i64, d: i64) -> i64 {
    assert!(0 <= c && c < m);
    assert!(0 <= d && d < m);
    if b == 0 || c == 0 {
        return a.max(0) * (n - 1);
    }
    // y = floor((cx + d) / m) <=> my <= cx + d < m(y + 1)
    // <=> (my - d) / c <= x < (my + m - d) / c
    // <=> floor((my - d + c - 1) / c) <= x <= floor((my + m - d - 1) / c)
    let ymax = (c * (n - 1) + d) / m;
    if a == 0 {
        return b.max(0) * ymax;
    }
    if a > 0 {
        return calc(ymax, c, b, a, m, m - d - 1).max(a * (n - 1) + b * ymax);
    }
    assert!(a < 0);
    return 0.max(calc(ymax, c, b, a, m, m + c - d - 1) + b);
}

fn calc(n: i64, m: i64, mut a: i64, b: i64, mut c: i64, mut d: i64) -> i64 {
    if n == 0 {
        return -1 << 60;
    }
    let mut bias = 0;
    {
        let q = quot(c, m);
        a += b * q;
        c -= q * m;
    }
    {
        let q = quot(d, m);
        bias += b * q;
        d -= q * m;
    }
    calc_inner(n, m, a, b, c, d) + bias
}

// https://yukicoder.me/problems/no/3397 (3.5)
fn main() {
    let t = getline().trim().parse::<i32>().unwrap();
    for _ in 0..t {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let [n, m, a, b, c, d] = ints[..] else { panic!() };
        println!("{}", calc(n, m, a, b, c, d));
    }
}
