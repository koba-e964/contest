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
    let p: i64 = get();
    let mut dp = vec![vec![0; n]; n];
    let mut ep = vec![vec![0; n]; n];
    dp[0][0] = 1;
    ep[0][1] = 1;
    for i in 1..n {
        for j in 0..n {
            let mut tmp = dp[i - 1][j] + ep[i - 1][j];
            if tmp >= p { tmp -= p; }
            if j >= 1 {
                tmp += dp[i - 1][j - 1] * 3;
            }
            if tmp >= p { tmp -= p; }
            if tmp >= p { tmp -= p; }
            if tmp >= p { tmp -= p; }
            dp[i][j] = tmp;
            let mut tmp = 0;
            if j >= 1 {
                tmp += ep[i - 1][j - 1];
            }
            if j >= 2 {
                tmp += dp[i - 1][j - 2] * 2;
            }
            if tmp >= p { tmp -= p; }
            if tmp >= p { tmp -= p; }
            ep[i][j] = tmp;
        }
    }
    for i in 1..n {
        print!("{}{}", dp[n - 1][i], if i + 1 == n { "\n" } else { " " });
    }
}
