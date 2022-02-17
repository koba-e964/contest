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

const MOD: i64 = 1_000_000_007;

fn main() {
    let n: usize = get();
    let mut dp = vec![[0; 16]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..16 {
            for k in 0..2 {
                dp[i + 1][(2 * j + k) % 16] += dp[i][j];
            }
        }
        dp[i + 1][10] = 0;
        for j in 0..16 {
            dp[i + 1][j] %= MOD;
        }
    }
    let mut ans = 1;
    for _ in 0..n {
        ans = ans * 2 % MOD;
    }
    for i in 0..16 {
        ans += MOD - dp[n][i];
        ans %= MOD;
    }
    println!("{}", ans);
}
