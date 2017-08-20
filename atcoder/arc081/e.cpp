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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;



int main(void) {
  string s;
  cin >> s;
  int n = s.length();
  // greedy
  VI dp(n + 2, 0);
  dp[n + 1] = 0;
  dp[n] = 1;
  vector<VI> next(n + 1, VI(26));
  set<char> seen;
  VI tbl(26, n + 1);
  for (int i = n - 1; i >= 0; --i) {
    tbl[s[i] - 'a'] = i + 1;
    int mi = 1e8;
    REP(j, 0, 26) {
      next[i][j] = tbl[j];
      mi = min(mi, dp[next[i][j]] + 1);
    }
    dp[i] = mi;
  }
  /*
  while (pos < n) {
    seen.insert(s[pos]);
    if (seen.size() == 26) {
      seen.clear();
      res += s[pos];
    }
    pos += 1;
  }
  for (char c = 'a'; c <= 'z'; ++c) {
    if (seen.count(c) == 0) {
      res += c;
      break;
    }
  }
  */
  string res;
  int cur = dp[0];
  int pos = 0;
  while (cur > 0) {
    REP(i, 0, 26) {
      char c = 'a' + i;
      if (dp[next[pos][i]] == cur - 1) {
	res += c;
	cur -= 1;
	pos = next[pos][i];
	break;
      }
    }
  }
  cout << res << endl;
}
