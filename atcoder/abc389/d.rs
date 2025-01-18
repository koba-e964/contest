fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn nth(a: i64, n: i64) -> i64 {
    let mut pass = 0;
    let mut fail = std::cmp::min(a, 1 << ((60 + n - 1) / n)) + 1;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        let mut tmp = 1i64;
        for _ in 0..n {
            tmp = tmp.saturating_mul(mid);
        }
        if tmp <= a {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    pass
}

fn main() {
    let n: i64 = getline().trim().parse().unwrap();
    let sq = |x: i64| x * x;
    let mut ans = 0;
    for i in 0..n {
        let tmp = nth(4 * n * n - sq(2 * i + 1), 2);
        if tmp == 0 {
            break;
        }
        let tmp = (tmp - 1) / 2;
        let mut add = 2 * tmp + 1;
        if i != 0 {
            add *= 2;
        }
        ans += add;
    }
    println!("{ans}");
}
