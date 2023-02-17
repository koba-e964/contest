fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

const MOD: i64 = 998_244_353;

fn main() {
    let s: Vec<_> = getline().trim().bytes().collect();
    let n = s.len();
    let mut dp = [0; 4];
    let t = b"101";
    dp[0] = 1;
    for i in 0..n {
        let mut ep = dp;
        if s[i] == b'?' {
            for j in 0..4 {
                ep[j] *= 2;
                if ep[j] >= MOD {
                    ep[j] -= MOD;
                }
            }
        }
        for j in 0..3 {
            if s[i] != t[j] {
                ep[j + 1] += dp[j];
                if ep[j + 1] >= MOD {
                    ep[j + 1] -= MOD;
                }
            }
        }
        dp = ep;
    }
    println!("{}", dp[3]);
}
