fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

// https://yukicoder.me/problems/no/3329 (3)
fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [n, maxw] = ints[..] else { panic!() };
    let v = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let w = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut dp = vec![vec![0; maxw + 1]; n + 1];
    for i in (0..n).rev() {
        for j in 0..maxw + 1 {
            dp[i][j] = dp[i + 1][j];
            if j >= w[i] {
                dp[i][j] = dp[i][j].max(dp[i + 1][j - w[i]] + v[i]);
            }
        }
    }
    let mut cur = maxw;
    let mut ans = vec![];
    for i in 0..n {
        if dp[i][cur] == dp[i + 1][cur] {
            continue;
        }
        cur -= w[i];
        ans.push(i);
    }
    println!("{}", ans.len());
    for i in 0..ans.len() {
        print!("{} ", ans[i] + 1);
    }
    println!();
}
