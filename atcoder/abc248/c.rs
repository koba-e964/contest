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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn main() {
    let n: usize = get();
    let m: usize = get();
    let k: usize = get();
    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    const MOD: i64 = 998_244_353;
    for _ in 0..n {
        let mut ep = vec![0; k + 1];
        for i in 1..m + 1 {
            for j in i..k + 1 {
                ep[j] += dp[j - i];
                ep[j] %= MOD;
            }
        }
        dp = ep;
    }
    let mut ans = 0;
    for i in 0..k + 1 {
        ans = (ans + dp[i]) % MOD;
    }
    println!("{}", ans);
}
