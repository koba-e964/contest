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

// https://yukicoder.me/problems/no/2846 (3)
// irb(main):009> [24,22,21,20,18,16,15,14,13,12,11,10,9,8,7,6,5].reduce{|a,b|a.lcm(b)}
// => 720720
// 素数 23, 19, 17 だけ特別扱いして、あとは幅 720720 の DP で解く。
fn main() {
    let k: usize = get();
    let n: usize = get();
    const W: usize = 720_720;
    let prs = [23, 19, 17];
    let mut dp = vec![0i64; W + 1];
    dp[0] = 1;
    for _ in 0..k {
        let mut ep = vec![0; W + 1];
        for i in 1..n + 1 {
            if W % i != 0 { continue; }
            let x = W / i;
            for j in x..W + 1 {
                ep[j] += dp[j - x];
            }
        }
        dp = ep;
    }
    let mut ans = dp[W];
    for &p in &prs {
        if k == p {
            ans += 1;
        }
    }
    println!("{}", ans);
}
