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
const ll mod = 1e9 + 7;

const int N = 123456;
VI edges[N];
int n, m;
int k, x;

ll dp[N][3][11];

void addup(ll &x, ll y) {
  x = (x + y) % mod;
}
void mulup(ll &x, ll y) {
  x = (x * y) % mod;
}

void poly_mul(ll *vec, const ll *obj) {
  VL ret(x + 1);
  REP(i, 0, x + 1) {
    REP(j, 0, x - i + 1) {
      addup(ret[i + j], vec[i] * obj[j] % mod);
    }
  }
  REP(i, 0, x + 1) {
    vec[i] = ret[i];
  }
}

void dfs(int v, int p) {
  dp[v][0][0] = k - 1;
  dp[v][1][1] = 1;
  dp[v][2][0] = m - k;
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    if (p == w) { continue; }
    dfs(w, v);
    // < k
    VL tmp(x + 1, 0);
    REP(i, 0, x + 1) {
      REP(b, 0, 3) {
	addup(tmp[i], dp[w][b][i]);
      }
    }
    poly_mul(dp[v][0], &tmp[0]);
    // = k
    poly_mul(dp[v][1], dp[w][0]);
    // > k
    fill(tmp.begin(), tmp.end(), 0);
    REP(i, 0, x + 1) {
      addup(tmp[i], dp[w][0][i]);
      addup(tmp[i], dp[w][2][i]);
    }
    poly_mul(dp[v][2], &tmp[0]);
  }
  if (0) {
    cerr << "dp[" << v << "]:"<<endl;
    REP(i, 0, x + 1) {
      cerr << " " << i << ":";
      REP(b, 0, 3) {
	cerr << " " << dp[v][b][i];
      }
      cerr << endl;
    }
  }
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n >> m;
  REP(i, 0, n - 1) {
    int u, v;
    cin >> u >> v;
    u--, v--;
    edges[u].push_back(v);
    edges[v].push_back(u);
  }
  cin >> k >> x;
  dfs(0, -1);
  ll sum = 0;
  REP(i, 0, 3) {
    REP(j, 0, x + 1) {
      sum += dp[0][i][j];
      sum %= mod;
    }
  }
  cout << sum << "\n";
}
