#include <iostream>
#include <string>
#include <algorithm>
#include <unordered_map>
#include <cassert>
#include <utility>
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

const int N = 300100;
int x[N], y[N];
VI g[N];
int diam[N];

bool vis[N];

void dfs(int v, int par, vector<PI> &tbl, int d = 0) {
  tbl.push_back(PI(v, d));
  REP(i, 0, g[v].size()) {
    int w = g[v][i];
    if (par == w) continue;
    dfs(w, v, tbl, d + 1);
  }
}

void mark(int v) {
  if (vis[v]) return;
  vis[v] = 1;
  REP(i, 0, g[v].size()) mark(g[v][i]);
}
int main(void) {
  cin.tie(0);
  ios_base::sync_with_stdio(false);
  int n, m, q;
  cin >> n >> m >> q;
  UnionFind uf(n);
  REP(i, 0, m) {
    cin >> x[i] >> y[i];
    x[i]--, y[i]--;
    uf.unite(x[i], y[i]);
    g[x[i]].push_back(y[i]);
    g[y[i]].push_back(x[i]);
  }
  REP(i, 0, n) diam[i] = -1;
  REP(i, 0, n) {
    vector<PI> tbl;
    if (vis[i]) continue;
    dfs(i, -1, tbl);
    PI ma(-1, -1);
    for (auto e: tbl) {
      ma = max(ma, PI(e.second, e.first));
    }
    tbl.clear();
    dfs(ma.second, -1, tbl);
    ma = PI(-1, -1);
    for (auto e: tbl) {
      ma = max(ma, PI(e.second, e.first));
    }
    diam[uf.root(i)] = ma.first;
    mark(i);
  }
  REP(i, 0, q) {
    int ty;
    cin >> ty;
    if (ty == 1) {
      int a;
      cin >> a;
      a--;
      cout << diam[uf.root(a)] << "\n";
    } else {
      int a, b;
      cin >> a >> b;
      a--, b--;
      int ra = uf.root(a);
      int rb = uf.root(b);
      if (ra == rb) continue;
      uf.unite(ra, rb);
      int da = diam[ra];
      int db = diam[rb];
      assert (da >= 0);
      assert (db >= 0);
      int newd = max(1 + (da + 1) / 2 + (db + 1) / 2, max(da, db));
      int newr = uf.root(ra);
      if (newr == ra) {
	diam[ra] = newd;
	diam[rb] = -1;
      } else {
	diam[rb] = newd;
	diam[ra] = -1;
      }
    }
    if (0) {
      cerr << i << ":";
      REP(i, 0, n) {
	cerr << " " << diam[i];
      }
      cerr << endl;
    }
  }
}
