use std::io::Read;

#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

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
    let a: usize = get();
    let b: usize = get();
    let n: usize = get();
    let mut banned = vec![vec![false; b]; a];
    for _ in 0..n {
        let x: usize = get();
        let y: usize = get();
        banned[x - 1][y - 1] = true;
    }
    let mut dp = vec![vec![0; b]; a];
    dp[0][0] = 1;
    for i in 0..a {
        for j in 0..b {
            if i + j == 0 || banned[i][j] {
                continue;
            }
            if i > 0 {
                dp[i][j] += dp[i - 1][j];
            }
            if j > 0 {
                dp[i][j] += dp[i][j - 1];
            }
        }
    }
    println!("{}", dp[a - 1][b - 1]);
}
