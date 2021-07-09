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

const MOD: i64 = 998_244_353;

fn main() {
    let n: usize = get();
    let k: usize = get();
    let mut dp = vec![vec![0; n + 1]; n + 1];
    dp[0][0] = 1;
    for i in 1..n + 1 {
        for j in (1..n + 1).rev() {
            dp[i][j] = dp[i - 1][j - 1];
            if 2 * j <= n {
                dp[i][j] += dp[i][2 * j];
            }
            dp[i][j] %= MOD;
        }
    }
    println!("{}", dp[n][k]);
}
