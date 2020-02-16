/**
 * Longest common subsequence. O(nm) where n = a.length(), m = b.length().
 * Verified by: RookieRank 2 - HackerRank in a String!
 * (https://www.hackerrank.com/contests/rookierank-2/challenges/hackerrank-in-a-string)
 * Header requirement: string, vector
 */
int lcs(const std::string &a, const std::string &b) {
  int n = a.length();
  int m = b.length();
  std::vector<std::vector<int> > dp(n + 1, std::vector<int>(m + 1, 0));
  for (int i = 0; i < n; ++i) {
    for (int j = 0; j < m; ++j) {
      int match = a[i] == b[j] ? 1 : 0;
      int ret = std::max(dp[i][j] + match, dp[i][j + 1]);
      ret = std::max(ret, dp[i + 1][j]);
      dp[i + 1][j + 1] = ret;
    }
  }
  return dp[n][m];
}
