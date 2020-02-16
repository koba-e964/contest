/**
 * Longest common subsequence. O(nm) where n = a.len(), m = b.len().
 * Verified by: RookieRank 2 - HackerRank in a String!
 * (https://www.hackerrank.com/contests/rookierank-2/challenges/hackerrank-in-a-string)
 */
fn lcs<T: PartialEq>(a: &[T], b: &[T]) -> usize {
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 0 .. n {
        for j in 0 .. m {
            let mat = if a[i] == b[j] { 1 } else { 0 };
            let ret = std::cmp::max(dp[i][j] + mat, dp[i][j + 1]);
            let ret = std::cmp::max(ret, dp[i + 1][j]);
            dp[i + 1][j + 1] = ret;
        }
    }
    dp[n][m]
}
