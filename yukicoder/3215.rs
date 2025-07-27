fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

const MOD: i64 = 998_244_353;

fn main() {
    let t: i32 = getline().trim().parse().unwrap();
    const N: usize = 200_100;
    const K: usize = 10;
    let mut pw = vec![0i64; N + 1];
    pw[0] = 1;
    for i in 1..=N {
        pw[i] = (pw[i - 1] * 2 * pw[i - 1]) % MOD;
    }
    let mut dp = vec![[0; K + 1]; N + 1];
    dp[1][1] = 1;
    dp[1][0] = 1;
    for i in 2..=N {
        let pre = i - 1;
        for j in 0..=K {
            for k in 0..=K - j {
                let val = dp[i][j + k] + dp[pre][j] * dp[pre][k];
                dp[i][j + k] = val % MOD;
            }
        }
        dp[i][0] += pw[i - 1] * pw[i - 1];
        dp[i][0] %= MOD;
    }
    for _ in 0..t {
        let ints = getline()
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let n = ints[0];
        let k = ints[1];
        println!("{}", dp[n][k - 1]);
    }
}
