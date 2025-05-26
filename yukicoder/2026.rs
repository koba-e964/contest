fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

const INF: i64 = 1 << 60;
// https://yukicoder.me/problems/no/2026 (4)
// O(N^2 log^2 N)
fn main() {
    let n: usize = getline().trim().parse().unwrap();
    let mut cv = vec![];
    for _ in 0..n {
        let v: Vec<i64> = getline().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        cv.push((v[0] as usize, v[1]));
    }
    let mut dp = vec![vec![-INF; n + 1]; n + 1];
    dp[0][0] = 0;
    for i in (1..=n).rev() {
        let (c, v) = cv[i - 1];
        let mut c = c.min(n / i);
        // decompose
        let mut w = vec![];
        let mut cur = 1;
        while c > 0 {
            let r = cur.min(c);
            w.push(r);
            c -= r;
            cur *= 2;
        }
        for w in w {
            for j in (i * w..=n).rev() {
                for k in 0..=(j / i).min(n - w) {
                    dp[k + w][j] = dp[k + w][j].max(dp[k][j - i * w] + v * w as i64);
                }
            }
        }
    }
    for k in 1..=n {
        let mut ans = -INF;
        for j in 0..=n {
            ans = ans.max(dp[k][j]);
        }
        println!("{ans}");
    }
}
