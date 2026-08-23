fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

// ret[i] = max {|L| : L is an increasing subsequence that ends at a[i] w.r.t cmp}
// If cmp = i64::lt, this function finds (usual) increasing subsequences.
// If cmp = i64::le, this function finds non-decreasing subsequences.
// Complexity: O(n log n)
// Verified by: https://atcoder.jp/contests/past202112-open/submissions/28433411
fn lis_by<F: FnMut(&i64, &i64) -> bool>(a: &[i64], mut cmp: F) -> Vec<usize> {
    let n = a.len();
    const INF: i64 = 1 << 60; // change here
    let mut dp = vec![INF; n + 1];
    let mut ans = vec![0; n];
    dp[0] = -INF;
    for i in 0..n {
        let mut pass = 0;
        let mut fail = n;
        while fail - pass > 1 {
            let mid = (fail + pass) / 2;
            if cmp(&dp[mid], &a[i]) {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        ans[i] = pass + 1;
        dp[pass + 1] = dp[pass + 1].min(a[i]);
    }
    ans
}

// 45度回転すれば、 (x1,y1)<=(x2,y2) := x1 <= x2 && y1 <= y2 という推移的DAGの上で最小パス被覆を求める問題になる。
// Dilworth の定理より、これは最大独立集合の大きさに等しい。
// これは LIS (の逆)になる。同じ x に対しては y の昇順に並べた上で、yの最長減少列を求めれば良い。
// - LIS を貼る
// - (x, y) を計算する
// - (x, y) でソートする
// - y だけ取り出して (-1) 倍する
// - LIS を呼ぶ
fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    let mut xy = vec![];
    for _ in 0..n {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let [t, x] = ints[..] else { panic!() };
        xy.push((t + x, t - x));
    }
    xy.sort();
    let mut ys = vec![];
    for (_, y) in xy {
        ys.push(-y);
    }
    let ans = lis_by(&ys, i64::lt);
    println!("{}", ans.iter().max().unwrap());
}
