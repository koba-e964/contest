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
        dp[pass + 1] = std::cmp::min(dp[pass + 1], a[i]);
    }
    ans
}
