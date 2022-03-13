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
    const MOD: i64 = 998_244_353;
    let mut dp = vec![[0; 9]; n];
    for i in 0..9 {
        dp[0][i] = 1;
    }
    for i in 1..n {
        for j in 0..9 {
            let mut me = dp[i - 1][j];
            if j > 0 {
                me += dp[i - 1][j - 1];
            }
            if j < 8 {
                me += dp[i - 1][j + 1];
            }
            dp[i][j] = me % MOD;
        }
    }
    let mut tot = 0;
    for i in 0..9 {
        tot = (tot + dp[n - 1][i]) % MOD;
    }
    println!("{}", tot);
}
