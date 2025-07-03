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

// https://yukicoder.me/problems/no/3102 (3)
fn main() {
    let t: usize = getline().trim().parse().unwrap();
    for _ in 0..t {
        let n: i64 = getline().trim().parse().unwrap();
        let s = nth(n, 2);
        let mut ans = -1;
        for x in 0.max(s - 3)..s + 3 {
            let rem = n ^ x;
            if rem >= x * x && rem < (x + 1) * (x + 1) {
                ans = rem;
                break;
            }
        }
        println!("{ans}");
    }
}
