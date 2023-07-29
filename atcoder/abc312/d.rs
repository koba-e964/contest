fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

const MOD: i64 = 998_244_353;

fn main() {
    let s: Vec<_> = getline().trim().chars().collect();
    let n = s.len();
    if n % 2 != 0 {
        println!("0");
        return;
    }
    let mut dp = vec![0; n / 2 + 1];
    dp[0] = 1;
    for i in 0..n {
        let mut ep = vec![0; n / 2 + 1];
        if s[i] != ')' {
            for j in 0..n / 2 {
                ep[j + 1] += dp[j];
            }
        }
        if s[i] != '(' {
            for j in 0..n / 2 {
                ep[j] += dp[j + 1];
            }
        }
        for v in &mut ep {
            *v %= MOD;
        }
        dp = ep;
    }
    println!("{}", dp[0]);
}
