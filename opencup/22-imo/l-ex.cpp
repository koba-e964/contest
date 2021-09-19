#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

void all_good(int n, string &s, vector<string> &ans) {
  if ((int) s.size() >= n) {
    ans.push_back(s);
    return;
  }
  int pos = s.size();
  REP(c, 'A', 'C' + 1) {
    if (pos >= 1 && s[pos - 1] == c) {
      continue;
    }
    s += c;
    all_good(n, s, ans);
    s.pop_back();
  }
}

bool is_awesome(int n, const string &a, const string &b) {
  int len = 2 * n + 1;
  assert((int) a.size() == len);
  assert((int) b.size() == len);
  vector<VI> dp(2 * n + 2, VI(2 * n + 2, 0));
  REP(i, 0, 2 * n + 2) {
    REP(j, 0, 2 * n + 2) {
      if (dp[i][j] > n) return 0;
      if (i < len) {
        dp[i + 1][j] = max(dp[i + 1][j], dp[i][j]);
      }
      if (j < len) {
        dp[i][j + 1] = max(dp[i][j + 1], dp[i][j]);
      }
      if (i < len && j < len) {
        int val = a[i] == b[j] ? 1 : 0;
        dp[i + 1][j + 1] = max(dp[i + 1][j + 1], dp[i][j] + val);
      }
    }
  }
  assert (dp[len][len] >= n);
  return dp[len][len] == n;
}

int test_with(int n, bool debug = 0) {
  vector<string> all;
  string s = "";
  all_good(2 * n + 1, s, all);
  cerr << "|good| = " << all.size() << endl;
  int ans = 0;
  REP(i, 0, all.size()) {
    if (i % 100 == 0) cerr << "i = " << i << endl;
    string t = all[i];
    for (auto u: all) {
      if (is_awesome(n, t, u)) {
        if (debug) cerr << t << " " << u << endl;
        ans++;
      }
    }
  }
  return ans;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n = 1;
  test_with(1, true);
  ll res = test_with(6);
  cerr << "n = " << 6 << ", awesome: " << res << endl;
  return 0;
  while (n < 6) {
    ll res = test_with(n);
    cerr << "n = " << n << ", awesome: " << res << endl;
    cerr << endl;
    n++;
  }
}
