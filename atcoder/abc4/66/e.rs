fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [n, k] = ints[..] else { panic!() };
    let mut a = vec![];
    let mut tot = 0;
    for _ in 0..n {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        a.push(ints[1] - ints[0]);
        tot += ints[0];
    }
    const INF: i64 = 1 << 50;
    let mut dp = vec![vec![-INF; 2 * k + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..n + 1 {
        for j in 0..2 * k + 1 {
            let mut me = dp[i - 1][j];
            if j > 0 {
                me = me.max(dp[i - 1][j - 1]);
            }
            dp[i][j] = me + if j % 2 == 1 { a[i - 1] } else { 0 };
        }
    }
    println!("{}", dp[n].iter().max().unwrap() + tot);
}
