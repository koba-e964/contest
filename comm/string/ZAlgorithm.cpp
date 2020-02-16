/*
 * Z algorithm. Calculates an array a[i] = |lcp(s, s[i...|s|])|,
 * where s is the given string.
 * If n = s.length(), the returned array has length n + 1.
 * E.g. z_algorithm("ababa") = {5, 0, 3, 0, 1, 0}
 * Reference: http://snuke.hatenablog.com/entry/2014/12/03/214243
 * Header Requirement: vector, string
 * Verified by: AtCoder ARC055-C (http://arc055.contest.atcoder.jp/submissions/950597)
 */
std::vector<int> z_algorithm(const std::string &s) {
  int n = s.length();
  std::vector<int> ret(n + 1);
  ret[0] = n;
  for (int i = 1, j = 0; i < n;) {
    for (; i + j < n && s[j] == s[i + j]; ++j) {}
    ret[i] = j;
    if (j == 0) { ++i; continue; }
    int k = 1;
    for (; i + k < n && k + ret[k] < j; ++k) {
      ret[i + k] = ret[k];
    }
    i += k; j -= k;
  }
  return ret;
}
