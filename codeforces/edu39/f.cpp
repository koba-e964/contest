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

const int DEBUG = 0;

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

ll powmod(ll a, ll e) {
  ll sum = 1;
  ll cur = a;
  while (e > 0) {
    if (e % 2) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}

void add(ll &x, ll y) {
  x = (x + y) % mod;
}

const int N = 103;
ll dp[4][N][N][N]; // 0: LR, 1: L, 2: R, 3: none

ll plen[N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, x;
  string s;
  cin >> n >> x >> s;
  plen[0] = plen[1] = 2;
  REP(i, 2, x + 1) {
    plen[i] = (plen[i - 1] * plen[i - 2]) % mod;
  }
  REP(i, 0, 2) {
    REP(j, 0, 4) {
      REP(l, 0, n) {
	dp[j][i][l][l] = s[l] == ('0' + i) ? 1 : 0;
      }
    }
  }
  REP(i, 2, x + 1) {
    REP(l, 0, n) {
      REP(r, l, n) {
	REP(kind, 0, 4) {
	  ll tmp = 0;
	  add(tmp, dp[kind][i - 1][l][r] * ((kind & 1) == 0 ? plen[i - 2] : 1));
	  add(tmp, dp[kind][i - 2][l][r] * ((kind & 2) == 0 ? plen[i - 1] : 1));
	  REP(mid, l + 1, r + 1) {
	    ll tmp2 = dp[kind | 1][i - 1][l][mid - 1];
	    tmp2 = tmp2 * dp[kind | 2][i - 2][mid][r] % mod;
	    add(tmp, tmp2);
	  }
	  dp[kind][i][l][r] = tmp;
	  if (DEBUG && kind == 0) {
	    DEBUGP(kind);
	    DEBUGP(i);
	    DEBUGP(l);
	    DEBUGP(r);
	    DEBUGP(dp[kind][i][l][r]);
	    cerr<<endl;
	  }
	}
      }
    }
  }
  cout << dp[0][x][0][n - 1] << "\n";
}
