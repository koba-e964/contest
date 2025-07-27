fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

const MOD: i64 = 998_244_353;

fn calc(n: usize, k: usize) -> i64 {
    let mut dp = vec![vec![0; n + 1]; n + 1];
    dp[0][0] = 1;
    for i in 0..=n {
        for j in 0..=i {
            if i == 0 && j == 0 {
                continue;
            }
            if i - j > k {
                continue;
            }
            let mut me = 0;
            if j > 0 {
                me += dp[i][j - 1];
            }
            if i > 0 {
                me += dp[i - 1][j];
            }
            dp[i][j] = me % MOD;
        }
    }
    dp[n][n]
}

fn main() {
    let ints = getline()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let n = ints[0];
    let k = ints[1];
    let ans = calc(n, k) - calc(n, k - 1) + MOD;
    println!("{}", ans % MOD);
}
