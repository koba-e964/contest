#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>
#include <cassert>
#include <map>
using namespace std;

const int DEBUG = 0;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;

const int T = 1000100;
VI occ[T];

const ll inf = 1e18;

int main(void) {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI t(n); REP(i, 0, n) cin >> t[i];
  VL v(n); REP(i, 0, n) cin >> v[i];
  REP(i, 0, n) {
    occ[t[i]].push_back(i);
  }
  VI pre(n, -1);
  REP(i, 0, T) {
    int sz = occ[i].size();
    if (sz == 0) continue;
    REP(j, 0, sz - 1) {
      pre[occ[i][j + 1]] = occ[i][j];
    }
  }
  REP(i, 1, n) {
    pre[i] = max(pre[i], pre[i - 1]);
  }
  if (DEBUG) {
    cerr << "pre:";
    REP(i, 0, n) cerr << " " << pre[i];
    cerr << endl;
  }
  map<ll, int> cont;
  VL dp(n + 1, inf);
  dp[0] = 0;
  REP(i, 0, n) {
    int p = pre[i];
    map<ll, int> nxt;
    for (auto e: cont) {
      ll key = e.first | v[i];
      if (nxt.count(key)) {
        nxt[key] = min(nxt[key], e.second);
      } else {
        nxt[key] = e.second;
      }
    }
    if (nxt.count(v[i]) == 0) {
      nxt[v[i]] = i;
    }
    cont = nxt;
    for (auto e: cont) {
      ll key = e.first;
      int start = max(e.second, p + 1);
      dp[i + 1] = min(dp[i + 1], dp[start] + key);
    }
    if (DEBUG) {
      cerr << "cont:";
      for (auto e: cont) {
        cerr << e.first << " [" << e.second << "," << i << "],";
      }
      cerr << endl;
      cerr << "dp[" << i + 1 << "]=" << dp[i + 1] << endl;
    }
  }
  cout << dp[n] << endl;
}
