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

const int K = 8;
const int M = 128;
const int N = 1010;
int n;
int a[N];
int dp[2][1 << K][K][M];

const int DEBUG = 0;

void chmax(int &u, int v) { u = max(u, v); }

int check(int x) {
  if (DEBUG) { cerr << "x = " << x << endl; }
  // only x and x + 1 continuous elements are taken into account
  REP(l, 0, 2) {
    REP(bits, 0, 1 << K) {
      REP(i, 0, K) {
	REP(j, 0, M) { dp[l][bits][i][j] = -1; }
      }
    }
  }
  REP(i, 1, n + 1) {
    int t = i % 2;
    REP(bits, 0, 1 << K) {
      REP(i, 0, K) {
	REP(j, 0, M) { dp[t][bits][i][j] = -1; }
      }
    }
    int j = a[i - 1];
    chmax(dp[t][1 << j][j][1], 1);
    REP(bits, 0, 1 << K) {
      REP(l, 0, x + 2) {
	REP(p, 0, K) {
	  chmax(dp[t][bits][p][l], dp[1-t][bits][p][l]);
	}
      }
      if ((bits & (1 << j)) == 0) { continue; }
      REP(l, 0, x + 2) {
	if (l >= 2 && dp[1-t][bits][j][l - 1] >= 0) {
	  chmax(dp[t][bits][j][l], dp[1-t][bits][j][l - 1] + 1);
	}
	if (l >= x && l <= x + 1) {
	  REP(p, 0, K) {
	    if (dp[1-t][bits ^ 1 << j][p][l] >= 0) {
	      chmax(dp[t][bits][j][1],
		    dp[1-t][bits ^ 1 << j][p][l] + 1);
	    }
	  }
	}
      }
    }
    if (DEBUG) {
      REP(bits, 0, 1 << K) {
	REP(j, 0, K) {
	  REP(l, 0, x + 2) {
	    int ret = dp[t][bits][j][l];
	    if (ret >= 0) {
	      cerr << "dp[" << i << "," << bits
		   << "," << j << "," << l << "]=" << ret << endl;
	    }
	  }
	}
      }
    }
  }
  int ma = -1;
  if (x == 0) {
    REP(bits, 0, 1 << K) {
      REP(j, 0, K) {
	REP(l, x, x + 2) {
	  ma = max(ma, dp[n%2][bits][j][l]);
	}
      }
    }
  } else {
    REP(j, 0, K) {
      REP(l, x, x + 2) {
	ma = max(ma, dp[n%2][(1 << K) - 1][j][l]);
      }
    }
  }
  return ma;
}

int main(void){
  cin >> n;
  REP(i, 0, n) {
    cin >> a[i];
    a[i]--;
  }
  int lo = 0;
  int hi = M - 2;
  while (hi - lo > 1) {
    int mid = (lo + hi) / 2;
    if (check(mid) < 0) {
      hi = mid;
    } else {
      lo = mid;
    }
  }
  cout << check(lo) << endl;
}
