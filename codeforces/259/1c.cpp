#include <iostream>
#include <string>
#include <algorithm>
#include <cassert>
#include <utility>
#include <set>
#include <vector>

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

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const int N = 100100;
int n;
VI g[N];
int x[N];

VI path;

void dfs(int v, int par) {
  path.push_back(v);
  x[v] = 1 - x[v];
  for (int w: g[v]) {
    if (par == w) continue;
    dfs(w, v);
    path.push_back(v);
    x[v] = 1 - x[v];
    if (x[w] == 1) {
      path.push_back(w);
      path.push_back(v);
      x[w] = 0;
      x[v] = 1 - x[v];
    }
  }
}


int main(void) {
  int m;
  cin >> n >> m;
  UnionFind uf(n);
  REP(i, 0, m) {
    int u, v;
    cin >> u >> v;
    u--, v--;
    if (uf.is_same_set(u, v)) continue;
    uf.unite(u, v);
    g[u].push_back(v);
    g[v].push_back(u);
  }
  vector<PI> pool;
  REP(i, 0, n) {
    cin >> x[i];
    if (x[i] == 1) {
      pool.push_back(PI(i, uf.root(i)));
    }
  }
  // !unique?
  {
    set<int> tt;
    REP(i, 0, pool.size()) {
      tt.insert(pool[i].second);
    }
    if (tt.size() >= 2) {
      cout << -1 << endl;
      return 0;
    }
  }
  // possible!
  if (pool.size() == 0) {
    cout << 0 << endl;
    return 0;
  }
  int idx = pool[0].first; // Any one is fine
  dfs(idx, -1);
  if (x[idx] == 1) {
    x[idx] = 0;
    assert (path.size() >= 1);
    assert (path[0] == idx);
    path.erase(path.begin());
  }
  REP(i, 0, n) assert (x[i] == 0);
  cout << path.size() << endl;
  REP(i, 0, path.size()) {
    cout << path[i] + 1 << (i == (int) path.size() - 1 ? "\n" : " ");
  }
}
