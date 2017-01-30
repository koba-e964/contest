#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 60;

const ll minf = -0x31415926535897LL;;
const ll inf = 0x31415926535897LL;;

ll dp[N][N][N]; // dp[l][r][j]: range [l,r), j modifications. Correct? max.
ll dp2[N][N][N]; // dp[l][r][j]: range [l,r), j modifications. Correct? min.

bool is_dig(char x) {
  return '0' <= x && x <= '9';
}

void fail(void) {
  cout << "NG" << endl;
  exit(0);
}

const int DEBUG = 0;

int main(void){
  int k;
  string s;
  cin >> k >> s;
  int n = s.length();
  if (n % 2 == 0) {
    fail();
  }
  if (not is_dig(s[0])) {
    if (k == 0) {
      fail();
    }
    s[0] = '9';
    k--;
  }
  // DP
  REP(l, 0, N) {
    REP(r, 0, N) {
      REP(j, 0, N) {
	dp[l][r][j] = minf;
	dp2[l][r][j] = inf;
      }
    }
  }
  REP(i, 0, n) {
    dp[i][i + 1][0] = is_dig(s[i]) ? s[i] - '0' : minf;
    dp2[i][i + 1][0] = is_dig(s[i]) ? s[i] - '0' : inf;
    dp[i][i + 1][1] = 9;
    dp2[i][i + 1][1] = 0;
    REP(u, 2, k + 1) {
      dp[i][i + 1][u] = minf;
      dp2[i][i + 1][u] = inf;
    }
    if (DEBUG) {
      cerr << "dp[" << i << "][" << i+1 << "][" << 0 << "]=" <<
	dp[i][i+1][0] << endl;
      cerr << "dp2[" << i << "][" << i+1 << "][" << 0 << "]=" <<
	dp2[i][i+1][0] << endl;
    }

  }
  REP(ss, 1, n / 2 + 1) {
    int w = 2 * ss + 1;
    REP(i, 0, n + 1 - w) {
      // handle [i, i + w)
      int last_p = s[i + w - 1] != '+' ? 1 : 0; // last is needed to mod last to +
      int last_m = s[i + w - 1] != '-' ? 1 : 0; // last is needed to mod last to +
      if (DEBUG) {
	cerr << "last_p=" << last_p << endl;
	cerr << "last_m=" << last_m << endl;
      }
      REP(u, 0, k + 1) {
	ll mi = inf;
	ll ma = minf;
	REP(j, i, i + w) {
	  // [i,j),[j,i+w)
	  REP(v, 0, u + 1) {
	    if (u - v - last_p >= 0) {
	      mi = min(mi, dp2[i][j][v] + dp2[j][i+w-1][u - v - last_p]);
	      ma = max(ma, dp[i][j][v] + dp[j][i+w-1][u - v - last_p]);
	    }
	    if (u - v - last_m >= 0) {
	      mi = min(mi, dp2[i][j][v] - dp[j][i+w-1][u - v - last_m]);
	      ma = max(ma, dp[i][j][v] - dp2[j][i+w-1][u - v - last_m]);
	    }
	  }
	}
	dp[i][i+w][u] = ma;
	dp2[i][i+w][u] = mi;
	if (DEBUG) {
	  cerr << "dp[" << i << "][" << i+w << "][" << u << "]" <<
	    dp[i][i+w][u] << endl;
	  cerr << "dp2[" << i << "][" << i+w << "][" << u << "]" <<
	    dp2[i][i+w][u] << endl;
	}
      }
    }
  }
  ll ma = minf;
  REP(i, 0, k + 1) {
    ma = max(ma, dp[0][n][i]);
  }
  if (ma <= -1e8) fail();
  cout << "OK" << endl;
  cout << ma << endl;
}
