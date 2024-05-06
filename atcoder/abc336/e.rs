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

fn f(n: [usize; 15], d: usize) -> i64 {
    let mut dp = vec![vec![[0i64; 2]; d]; d + 1];
    dp[0][0][1] = 1;
    for i in 0..15 {
        let mut ep = vec![vec![[0i64; 2]; d]; d + 1];
        for j in 0..d + 1 {
            for k in 0..d {
                for eq in 0..2 {
                    let val = dp[j][k][eq];
                    for q in 0..10 {
                        if eq == 1 && q > n[i] {
                            break;
                        }
                        let neq = if eq == 1 && q == n[i] { 1 } else { 0 };
                        if j + q > d {
                            continue;
                        }
                        ep[j + q][(10 * k + q) % d][neq] += val;
                    }
                }
            }
        }
        dp = ep;
    }
    dp[d][0][0] + dp[d][0][1]
}

fn main() {
    let mut n: i64 = get();
    let mut dig = [0; 15];
    for i in 0..15 {
        dig[14 - i] = (n % 10) as usize;
        n /= 10;
    }
    let mut ans = 0;
    for d in 1..127 {
        ans += f(dig, d);
    }
    println!("{ans}");
}
