#include <algorithm>
#include <cassert>
#include <cstring>
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
const ll mod = 1e9 + 7;

ll fact[120];

void init(void) {
  fact[0] = 1;
  REP(i, 1, 120) {
    fact[i] = fact[i - 1] * i % mod;
  }
}

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

ll comb(ll x, ll y) {
  return fact[x] * powmod(fact[x - y] * fact[y] % mod, mod - 2) % mod;
}

const int DEBUG = 0;

const int N = 110;

VI edges[N];

ll dp[N][N][N];
ll dp2[N][N];


int dfs(int v, int p) {
  int tot = 1;
  dp[v][1][0] = 1;
  REP(i, 0, edges[v].size()) {
  if (DEBUG) {
    cerr << "dp[" << v << "]:"<<endl;
    REP(i, 0, tot + 1) {
      REP(j, 0, tot + 1) {
	cerr << " " << dp[v][i][j];
      }
      cerr << endl;
    }
  }
    int w = edges[v][i];
    if (w == p) continue;
    int sub = dfs(w, v);
    tot += sub;
    memset(dp2, 0, sizeof(dp2));
    REP(j, 0, tot - sub + 1) {
      REP(k, 0, tot - sub + 1) {
	if (dp[v][j][k] == 0) continue;
	REP(p, 0, sub + 1) {
	  REP(l, 0, sub + 1) {
	    dp2[j + l][k + p] += dp[v][j][k] * dp[w][l][p];
	    dp2[j + l][k + p] %= mod;
	    if (k + 1 + p < N) {
	      dp2[j][k + 1 + p] += dp[v][j][k] * dp[w][l][p] % mod * l;
	      dp2[j][k + 1 + p] %= mod;
	    }
	  }
	}
      }
    }
    swap(dp[v], dp2);
  }
  if (DEBUG) {
    cerr << "dp[" << v << "]:"<<endl;
    REP(i, 0, tot + 1) {
      REP(j, 0, tot + 1) {
	cerr << " " << dp[v][i][j];
      }
      cerr << endl;
    }
  }
  return tot;
}



// https://camypaper.bitbucket.io/2017/08/11/tco2014r3bh/
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  init();
  int n;
  cin >> n;
  REP(i, 0, n - 1) {
    int u, v;
    cin >> u >> v;
    u--, v--;
    edges[u].push_back(v);
    edges[v].push_back(u);
  }
  dfs(0, -1);
  VL b(n + 1);
  b[0] = 1;
  REP(i, 1, n + 1) {
    REP(j, 1, n + 1) {
      b[i] += j * (dp[0][j][i] * powmod(n, i - 1) % mod) % mod;
      b[i] %= mod;
    }
  }
  VL a(n + 1);
  REP(i, 0, n + 1) {
    a[i] = b[i];
    REP(j, 0, i) {
      a[i] += mod - comb(n - 1 - j, i - j) * a[j] % mod;
      a[i] %= mod;
    }
  }
  REP(i, 0, n) {
    cout << a[n - 1 - i] << (i == n - 1 ? "\n" : " ");
  }
}
