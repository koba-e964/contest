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

const int DEBUG = 0;

/*
 * Union-Find tree
 * header requirement: vector
 */
class UnionFind {
private:
  std::vector<int> disj;
  std::vector<int> rank;
public:
  UnionFind(int n) : disj(n), rank(n) {
    for (int i = 0; i < n; ++i) {
      disj[i] = i;
      rank[i] = 0;
    }
  }
  int root(int x) {
    if (disj[x] == x) {
      return x;
    }
    return disj[x] = root(disj[x]);
  }
  void unite(int x, int y) {
    x = root(x);
    y = root(y);
    if (x == y) {
      return;
    }
    if (rank[x] < rank[y]) {
      disj[x] = y;
    } else {
      disj[y] = x;
      if (rank[x] == rank[y]) {
	++rank[x];
      }
    }
  }
  bool is_same_set(int x, int y) {
    return root(x) == root(y);
  }
};


#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const int N = 200000;
ll beet[N + 1];

VI g[N];



int a[N];

bool vis[N];
int dfs(int v, int par, int d) {
  if (a[v] % d != 0) return 0;
  vis[v] = true;
  int u = 1;
  for (int w: g[v]) {
    if (par == w) continue;
    u += dfs(w, v, d);
  }
  return u;
}

VI inv[N + 1];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> a[i];
    inv[a[i]].push_back(i);
  }

  REP(i, 0, n - 1) {
    int x, y;
    cin >> x >> y;
    x--, y--;
    g[x].push_back(y);
    g[y].push_back(x);
  }
  REP(d, 1, N + 1) {
    VI activated;
    for (int i = d; i <= N; i += d) {
      for (int v: inv[i])
        activated.push_back(v);
    }
    if (activated.size() == 0) continue;
    ll tot = 0;
    for (int idx: activated) {
      if (vis[idx]) continue;
      ll cc = dfs(idx, -1, d);
      tot += cc * (cc + 1);
    }
    for (int idx: activated) vis[idx] = false;
    if (DEBUG) {
      DEBUGP(d);
      DEBUGP(tot / 2);
      cerr << endl;
    }
    beet[d] = tot / 2;
  }
  if (DEBUG) {
    REP(d, 1, 10) cerr << d << " ==> " << beet[d] << endl;
  }
  // incl-excl
  for (int d = N; d >= 1; --d) {
    for (int e = 2 * d; e <= N; e += d) {
      beet[d] -= beet[e];
    }
  }
  REP(d, 1, N + 1) {
    if (beet[d] != 0) {
      cout << d << " " << beet[d] << "\n";
    }
  }
}
