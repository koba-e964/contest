use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

// https://yukicoder.me/problems/no/2318 (3)
// N の約数は多くとも 6720 個程度。
fn main() {
    let n: i64 = get();
    let mut div = vec![];
    {
        let mut d = 1;
        while d * d <= n {
            if n % d == 0 {
                div.push(d);
                if n != d * d {
                    div.push(n / d);
                }
            }
            d += 1;
        }
    }
    div.sort();
    let m = div.len();
    let mut hm = std::collections::HashMap::new();
    for i in 0..m {
        let mut c = 0;
        for j in 0..i + 1 {
            if div[i] % div[j] == 0 {
                c += 1;
            }
        }
        hm.insert(div[i], c);
    }
    let mut dp = vec![0; m];
    dp[0] += 1;
    const MOD: i64 = 998_244_353;
    for i in 1..m {
        for j in 0..i {
            if div[i] % div[j] == 0 {
                // #{x | lcm(div[j], x) = div[i]} = #{x | lcm(div[j], x) = div[i]}
                let q = div[i] / div[j];
                let mut cap = div[i];
                loop {
                    let g = gcd(cap, q);
                    if g == 1 { break; }
                    cap /= g;
                }
                let coef = hm[&cap];
                dp[i] += dp[j] * coef;
            }
        }
        dp[i] %= MOD;
    }
    println!("{}", dp[m - 1]);
}
