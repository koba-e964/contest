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



using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;


class OnlySanta {
public:
  string solve(string s) {
    string t = "ST";
    if (lcs(s, "STAN") == 4) {
      return s + "TA";
    }
    int pos = 0;
    int cur = 0;
    VI tt;
    while (pos < 2) {
      int last = -1;
      REP(i, cur, s.length()) {
	if (s[i] == t[pos]) {
	  last = i;
	  break;
	}
      }
      if (last == -1) {
	break;
      }
      cur = last;
      tt.push_back(last);
      pos++;
    }
    if (tt.size() == 2) {
      int s1 = tt[0];
      cerr << s1 << endl;
      return s.substr(0, s1 + 1) + "AN" + s.substr(s1 + 1) + "A";
    }
    return s + "SANTA";
  }
};




int main(void) {
  string s;
  cin >> s;
  string ans = OnlySanta().solve(s);
  if (lcs(ans, "SANTA") != 5) {
    cerr << ans << " does not contain SANTA" << endl;
  }
  if (lcs(ans, "SATAN") == 5) {
    cerr << ans << " contains SATAN" << endl;
  }
  cout << ans << endl;
}
