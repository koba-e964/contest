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

// Tags: dp, subsequence-dp, trinity
fn main() {
    let n: usize = get();
    let s: Vec<_> = get_word().chars().collect();
    let same = s.iter().all(|&x| x == s[0]);
    if same {
        println!("1");
        return;
    }
    let alp = vec!['+', 'A', 'B', 'C'];
    let ind: Vec<_> = s.iter()
        .map(|&x| alp.iter().position(|&y| y == x).unwrap())
        .collect();
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] ^ ind[i];
    }
    let mut nxt = vec![[n + 1; 4]; n + 1];
    nxt[n][acc[n]] = n;
    for i in (0..n).rev() {
        nxt[i] = nxt[i + 1];
        nxt[i][acc[i]] = i;
    }
    let mut dp = vec![0; n + 1];
    dp[n] = 1;
    for i in (0..n).rev() {
        for j in 1..4 {
            let to = nxt[i][acc[i] ^ j];
            if to <= n {
                dp[i] = (dp[i] + dp[to]) % MOD;
            }
        }
        if acc[i] == acc[n] {
            dp[i] = (dp[i] + 1) % MOD;
        }
    }
    let mut tot = dp[0];
    if acc[n] == 0 {
        tot = (tot + MOD - 1) % MOD;
    }
    println!("{}", tot);
}
