fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    let mut d = vec![vec![]; n];
    for i in 0..n {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        d[i] = ints;
    }
    const INF: i32 = 1 << 27;
    let mut dp = vec![vec![INF; n]; 1 << n];
    dp[1][0] = 0;
    for bits in 2usize..1 << n {
        for i in 0..n {
            if (bits & 1 << i) == 0 { continue; }
            let mut me = INF;
            for j in 0..n {
                if i == j || (bits & 1 << j) == 0 { continue; }
                me = me.min(d[i][j] * (n as i32 - bits.count_ones() as i32 + 1) + dp[bits ^ 1 << i][j]);
            }
            dp[bits][i] = me;
        }
    }
    let mut ans = INF;
    for i in 0..n {
        ans = ans.min(dp[(1 << n) - 1][i]);
    }
    println!("{ans}");
}
