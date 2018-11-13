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
typedef pair<int, int> PI;

const int N = 300100;
VI g[N];

int dep[N];
int st[N], en[N];
int inv[N];
VI euler[N];
VL acc[N];
VL val[N];
ll dp[N];

void dfs(int v, int par, int &cnt, int d) {
  dep[v] = d;
  euler[d].push_back(cnt);
  st[v] = cnt;
  inv[cnt] = v;
  cnt++;
  for (int w: g[v]) {
    if (w == par) continue;
    dfs(w, v, cnt, d + 1);
  }
  en[v] = cnt;
}

void dfs2(int v, int par, ll acc) {
  dp[v] += acc;
  for (int w: g[v]) {
    if (w == par) continue;
    dfs2(w, v, dp[v]);
  }
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 0, n - 1) {
    int x, y;
    cin >> x >> y;
    x--, y--;
    g[x].push_back(y);
    g[y].push_back(x);
  }
  int cnt = 0;
  dfs(0, -1, cnt, 0);
  // initialise acc
  REP(i, 0, N) {
    int len = euler[i].size();
    acc[i] = VL(len + 1, 0);
    val[i] = VL(len, 0);
  }
  int m;
  cin >> m;
  REP(i, 0, m) {
    int v, d, x;
    cin >> v >> d >> x;
    v--;
    int dd = min(dep[v] + d + 1, N - 1);
    int lo = lower_bound(euler[dd].begin(), euler[dd].end(), st[v]) - euler[dd].begin();
    int hi = lower_bound(euler[dd].begin(), euler[dd].end(), en[v]) - euler[dd].begin();
    acc[dd][lo] -= x;
    acc[dd][hi] += x;
    int idx = lower_bound(euler[dep[v]].begin(), euler[dep[v]].end(), st[v]) - euler[dep[v]].begin();
    val[dep[v]][idx] += x;
  }
  REP(i, 0, N) {
    int len = euler[i].size();
    REP(j, 0, len) acc[i][j + 1] += acc[i][j];
    REP(j, 0, len) val[i][j] += acc[i][j];
    REP(j, 0, len) {
      dp[inv[euler[i][j]]] = val[i][j];
    }
  }
  dfs2(0, -1, 0);
  REP(i, 0, n) {
    cout << dp[i] << (i == n - 1 ? "\n" : " ");
  }
}
