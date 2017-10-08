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

const int DEBUG = 0;

int calc(const VI &v) {
  int n = v.size();
  if (n <= 1) {
    return 0;
  }
  if (n == 2) {
    return max(v[0], v[1]);
  }
  vector<VI> dp(n + 1, VI(3, -1e8));
  const int inf = 1e8;
  dp[0][0] = 0;
  REP(i, 0, n) {
    int ma = -inf;
    ma = max(ma, dp[i][1] + v[i] - 1);
    ma = max(ma, dp[i][2] + 1);
    ma = max(ma, dp[i][0]);
    dp[i + 1][0] = ma;

    ma = -inf;
    if (v[i] > 1) {
      ma = max(ma, dp[i][1] + v[i] - 1);
      ma = max(ma, dp[i][2] + 2);
    }
    ma = max(ma, dp[i][0] + 1);
    dp[i + 1][1] = ma;

    ma = -inf;
    if (v[i] > 1) {
      ma = max(ma, dp[i][2] + v[i] - 1);
      ma = max(ma, dp[i][1] + v[i] - 2);
    }
    ma = max(ma, dp[i][0] + v[i] - 1);
    dp[i + 1][2] = ma;
    if (DEBUG) {
      cerr << "dp[" << i+1 << "]";
      REP(j, 0, 3) {
	cerr << " " << dp[i+1][j];
      }
      cerr << endl;
    }
  }
  return dp[n][0];
}


int main(void) {
  int n;
  cin >> n;
  string s;
  cin >> s;
  vector<VI> seg;
  VI cur;
  int onelen = 0;
  REP(i, 0, s.length()) {
    if (s[i] == '0') {
      if (onelen == 0 && cur.size() > 0) {
	// cut
	seg.push_back(cur);
	cur.clear();
      } else {
	if (onelen > 0) {
	  cur.push_back(onelen);
	  onelen = 0;
	}
      }
    } else {
      onelen += 1;
    }
  }
  if (onelen > 0) {
    cur.push_back(onelen);
    onelen = 0;
  }
  if (cur.size() > 0) {
    seg.push_back(cur);
    cur.clear();
  }
  if (0) {
    cerr << "seg:" << endl;
    for (auto v: seg) {
      for (int t: v) {
	cerr << " " << t;
      }
      cerr << endl;
    }
  }
  int tot = 0;
  for (VI v: seg) {
    tot += calc(v);
  }
  cout << tot << endl;
}
