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

#if 1
const int B = 550;
const int N = B * B;
const int C = 24; /// C^D >= N
const int D = 4;
#else
const int B = 3;
const int N = B * B;
const int C = 2;
const int D = 3;
#endif


int dp[N];
int max_depth[N];

int n;
VI edges[N];

void dfs(int v, int par, int k) {
  VI pool;
  int ma = 1;
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    if (par == w) continue;
    dfs(w, v, k);
    pool.push_back(max_depth[w]);
    ma = max(ma, dp[w]);
  }
  max_depth[v] = 1;
  if ((int) pool.size() >= k) {
    nth_element(pool.begin(), pool.end() - k, pool.end());
    max_depth[v] = pool[pool.size() - k] + 1;
    ma = max(ma, max_depth[v]);
  }
  dp[v] = ma;
}

int dp_dep[D + 1][N]; // maximum k s.t. dp_k[u] >= d
int dp_dep_direct[D + 1][N]; // maximum k s.t. dp_k[u] >= d

int dfs_dep(int v, int par, int dep) {
  int ma = 0;
  VI pool;
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    if (par == w) continue;
    int val = dfs_dep(w, v, dep);
    pool.push_back(dp_dep_direct[dep - 1][w]);
    ma = max(ma, val);
  }
  int ma_dir = 0;
  sort(pool.rbegin(), pool.rend());
  REP(i, 0, pool.size()) {
    if (pool[i] >= i + 1) {
      ma_dir = max(ma_dir, i + 1);
    }
  }
  dp_dep_direct[dep][v] = ma_dir;
  return dp_dep[dep][v] = max(ma, ma_dir);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n;
  REP(i, 0, n - 1) {
    int u, v;
    cin >> u >> v;
    u--, v--;
    edges[u].push_back(v);
    edges[v].push_back(u);
  }
  ll tot = 0;
  // k <= C
  REP(k, 1, C + 1) {
    if (k <= n) {
      dfs(0, -1, k);
      REP(i, 0, n) {
	tot += dp[i];
      }
    }
  }
  // k > V
  if (n > C) {
    REP(i, 0, n) {
      dp_dep[1][i] = n;
      dp_dep_direct[1][i] = n;
    }
    REP(b, 2, D + 1) {
      dfs_dep(0, -1, b);
    }
    REP(i, 0, n) {
      if (0) {
	DEBUGP(i);
	REP(b, 2, D + 1) {
	  DEBUGP(b);
	  DEBUGP(dp_dep[b][i]);
	  DEBUGP(dp_dep_direct[b][i]);
	}
      }
      tot += n - C;
      REP(b, 2, D + 1) {
	tot += max(dp_dep[b][i], C) - C;
      }
    }
  }
  cout << tot << endl;
}
