fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

const MOD: i64 = 998_244_353;

fn main() {
    let s: Vec<_> = getline().trim().bytes().map(|b| (b - b'0') as usize).collect();
    let n = s.len();
    let mut dp = vec![[0; 2]; 1 << 10];
    for i in 0..n {
        let mut ep = vec![[0; 2]; 1 << 10];
        // start
        for d in 1..if i == 0 { s[0] + 1 } else { 10 } {
            ep[1 << d][if i == 0 && d == s[0] { 1 } else { 0 }] += 1;
        }
        // continue
        for bits in 0..1 << 10 {
            for eq in 0..2 {
                for d in 0..if eq == 1 { s[i] + 1 } else { 10 } {
                    let neq = if eq == 1 && d == s[i] { 1 } else { 0 };
                    ep[bits ^ 1 << d][neq] += dp[bits][eq];
                    ep[bits ^ 1 << d][neq] %= MOD;
                }
            }
        }
        dp = ep;
    }
    println!("{}", (dp[0][0] + dp[0][1]) % MOD);
}
