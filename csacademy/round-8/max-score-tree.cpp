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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;

const int N = 100100;
VI g[N];
ll a[N];

// 0: on a subtree, 1: tree ends here
ll dp[N][2];

const ll inf = 1e18;

void dfs(int v, int par) {
  VL sub;
  REP(i, 0, g[v].size()) {
    int w = g[v][i];
    if (w == par) continue;
    dfs(w, v);
    sub.push_back(dp[w][0]);
  }
  sort(sub.rbegin(), sub.rend());
  VL acc(sub.size() + 1);
  ll ma = -inf;
  REP(i, 0, sub.size()) {
    acc[i + 1] = acc[i] + sub[i];
  }
  REP(i, 0, sub.size() + 1) {
    ma = max(ma, acc[i] + a[i + 1]);
  }
  dp[v][0] = ma;
  ma = -inf;
  REP(i, 0, sub.size() + 1) {
    ma = max(ma, acc[i] + a[i]);
  }
  assert (ma >= a[0]);
  dp[v][1] = ma;
}

// D
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 0, N) a[i] = -inf;
  REP(i, 0, n) cin >> a[i];
  REP(i, 0, n - 1) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    g[a].push_back(b);
    g[b].push_back(a);
  }
  dfs(0, -1);
  ll ma = -inf;
  REP(i, 0, n) {
    ma = max(ma, dp[i][1]);
  }
  cout << ma << endl;
}
