#include <vector>
#include <string>
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

#include <iostream>
#include <cassert>
#include <algorithm>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void){
  string s;
  cin >> s;
  int n = s.length();
  string srev(s);
  reverse(srev.begin(), srev.end());
  VI z1 = z_algorithm(s);
  VI z2 = z_algorithm(srev);
  ll sum = 0;
  // try a partition s = x y
  REP(i, n / 2 + 1, n - 1) {
    assert (i > n - i);
    assert (i >= 2);
    // Find maximum a, c s.t. x[0...a] = y[0...a] && x[|x|-c...|x|] = y[|y|-c...|y|]
    int a = 0, c = 0;
    a = z1[i];
    c = z2[n - i];
    // #{(i, j) | 1 <= i <= a, 1 <= j <= c, i + j == n - i}
    a = min(a, n - i - 1);
    c = min(c, n - i - 1);
    int tmp = a == 0 || c == 0 ? 0 : a + c >= n - i ? a + c - n + i + 1 : 0;
    sum += tmp;
  }
  cout << sum << endl;
}
