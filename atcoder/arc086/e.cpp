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

const int N = 3010;
const int NN = 201000;
const int DEBUG = 0;



ll dp[N][2];
int p[NN];
VI ch[NN];


VL dat[NN];
VI nc[NN];

int dfs(int v, int d) {
  if (d == 0) {
    dp[v][0] = dp[v][1] = 1;
    return 1;
  }
  int nc = 0;
  ll a0 = 1, a1 = 0, a2 = 0;
  REP(i, 0, ch[v].size()) {
    int w = ch[v][i];
    nc += dfs(w, d - 1);
    ll b0 = a0 * dp[w][0] % mod;
    ll b1 = (a0 * dp[w][1] + a1 * dp[w][0]) % mod;
    ll b2 = (a1 * dp[w][1] + a2 * dp[w][0]) % mod;
    b2 = (b2 + a2 * dp[w][1]) % mod;
    a0 = b0;
    a1 = b1;
    a2 = b2;
  }
  dp[v][0] = (a0 + a2) % mod;
  dp[v][1] = a1;
  if (DEBUG && 0) {
    cerr << "dp["<<v<<"]:";
    cerr << dp[v][0] << " " << dp[v][1] << endl;
  }
  return nc;
}

int dfs2(int v) {
  if (ch[v].size() == 0) {
    dat[v] = VL(1, 1);
    nc[v] = VI(1, 1);
    return v;
  }
  if (ch[v].size() == 1) {
    int w = ch[v][0];
    int res = dfs2(w);
    dat[res].push_back(1);
    nc[res].push_back(1);
    return res;
  }
  vector<PI> pool;
  REP(i, 0, ch[v].size()) {
    int w = ch[v][i];
    int res = dfs2(w);
    pool.push_back(PI(nc[res].size(), res));
  }
  sort(pool.rbegin(), pool.rend());

  int tip = pool[0].second;
  bool single = false;
  REP(i, 0, nc[tip].size()) {
    int ifix = (nc[tip].size() - i - 1);
    int tot_nc = 0;
    ll tot = 0;
    ll prod = 1;
    REP(j, 0, pool.size()) {
      int jidx = pool[j].second;
      if ((int) nc[jidx].size() <= i) {
	if (j == 1) {
	  single = true;
	}
	break;
      }
      int k = nc[jidx].size() - i - 1;
      ll s = dat[jidx][k];
      ll pw = powmod(2, nc[jidx][k]);
      tot_nc += nc[jidx][k];
      pw = (pw - s + mod) % mod;
      tot += s * powmod(pw, mod - 2) % mod;
      tot %= mod;
      prod = prod * pw % mod;
    }
    tot = tot * prod % mod;
    if (DEBUG) {
      cerr << "v = " << v << ", i = "<< i << endl;
      cerr << "tot = " << tot << endl;
      cerr << "tot_nc = " << tot_nc << endl;
    }
    dat[tip][ifix] = tot;
    nc[tip][ifix] = tot_nc;
    if (single) {break;}
  }
  dat[tip].push_back(1);
  nc[tip].push_back(1);
  return tip;
}



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> p[i + 1];
    ch[p[i + 1]].push_back(i + 1);
  }
#if 0
  ll tot = 0;
  REP(d, 0, n + 1) {
    int nc = dfs(0, d);
    tot += dp[0][1] * powmod(2, n + 1 - nc) % mod;
    if (DEBUG && dp[0][1] != 0) {
      cerr << "d = " << d << endl;
      cerr << "nc(" << d << ") = " << nc << endl;
      cerr << "net: " << dp[0][1] << endl;
      cerr << "bias = " << powmod(2, n + 1 - nc) << endl;
      cerr << endl;
    }
    tot %= mod;
  }
  cout << tot << "\n";

  // O(n)
#endif
  int tip = dfs2(0);
  ll tot2 = 0;
  if (DEBUG) {
    cerr << "dat";
    for (auto v: dat[tip]) cerr << " " << v;
    cerr << endl << "nc";
    for (auto v: nc[tip]) cerr << " " << v;
    cerr << endl;
  }
  REP(i, 0, nc[tip].size()) {
    int nc2 = nc[tip][i];
    tot2 += dat[tip][i] * powmod(2, n + 1 - nc2) % mod;
    tot2 %= mod;
  }
  cout << tot2 << endl;
}
