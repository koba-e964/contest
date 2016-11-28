#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
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

typedef std::pair<ll, int> PLI;

/**
 * Lowest Common Ancestor. Call lca(x, y) to get the lca of them.
 * Header Requirement: vector, cassert
 * Verified by: AtCoder ABC014-D (http://abc014.contest.atcoder.jp/submissions/759125)
 */
class LowestCommonAncestor {
private:
  int n, bn;
  std::vector<PLI> parent; // 0 is root, parent[0] = 0
  std::vector<int> dep;
  
  // Lowest Common Ancestor
  
  std::vector<std::vector<PLI> > lca_tbl;
  
  void dfs(const std::vector<std::vector<PLI> > &edges, int v, int par, int d, ll x) {
    parent[v].second = par;
    parent[v].first = x;
    dep[v] = d;
    
    for (int i = 0; i < edges[v].size(); ++i) {
      int u = edges[v][i].second;
      if (u != par) {
	dfs(edges, u, v, d + 1, edges[v][i].first);
      }
    }
  }
  
  void lca_init(void) {
    for (int v = 0; v < n; ++v) {
      lca_tbl[v] = std::vector<PLI>(bn + 1, PLI(0, 0));
      lca_tbl[v][0] = parent[v];
    }
    for (int i = 1; i <= bn; ++i) {
      for (int v = 0; v < n; ++v) {
	PLI tmp = lca_tbl[v][i - 1];
        PLI tmp2 = lca_tbl[tmp.second][i - 1];
	lca_tbl[v][i] = PLI(max(tmp.first, tmp2.first), tmp2.second);
      }
    }
  }
public:
  ll lca(int x, int y) const {
    int dx = dep[x];
    int dy = dep[y];
    ll res = 0;
    if (dx > dy) {
      return lca(y, x);
    }
    // Go up from y to the depth of x
    for (int l = bn; l >= 0; --l) {
      if (dy - dx >= 1 << l) {
        PLI ny = lca_tbl[y][l];
	res = max(res, ny.first);
	y = ny.second;
	dy -= 1 << l;
      }
    }

    assert (dx == dy);

    if (x == y) {
      return res;
    }
  
    for (int l = bn; l >= 0; --l) {
      if (lca_tbl[x][l].second != lca_tbl[y][l].second) {
	PLI nx = lca_tbl[x][l];
	PLI ny = lca_tbl[y][l];
	res = max(res, max(nx.first, ny.first));
	x = nx.second;
	y = ny.second;
      }
    }
    return max(res, max(lca_tbl[x][0].first, lca_tbl[y][0].first));
  }
  int depth(int a) const {
    return dep[a];
  }
  LowestCommonAncestor(int n, const std::vector<std::vector<PLI> > &edges)
    : n(n), parent(n), dep(n), lca_tbl(n) {
    bn = 0;
    while (n > 1 << bn) {
      bn++;
    }
    dfs(edges, 0, 0, 0, 0);
    lca_init();
  }
};



const int M = 400010;
int n, m;
int a[M], b[M];
ll c[M];

const int N = 4010;
vector<PLI> edges[N];

ll dfs(int v, int p, int t) {
  if (v == t) {
    return 0;
  }
  REP(i, 0, edges[v].size()) {
    PLI e = edges[v][i];
    int w = e.second;
    if (p == w) continue;
    ll res = dfs(w, v, t);
    if (res == -1) continue;
    return max(res, e.first);
  }
  return -1;
}

ll solve(int s, int t) {
  return dfs(s, -1, t);
  
}

int main(void){
  cin >> n >> m;
  REP(i, 0, m) {
    cin >> a[i] >> b[i] >> c[i];
    a[i]--, b[i]--;
  }
  int q;
  cin >> q;
  vector<PLI> pool;
  ll tot = 0;
  REP(i, 0, m) {
    pool.push_back(PLI(c[i], i));
  }
  sort(pool.begin(), pool.end());
  UnionFind uf(n);
  vector<vector<PLI> > edges_i(n);
  REP(i, 0, pool.size()) {
    int idx = pool[i].second;
    int u = a[idx];
    int v = b[idx];
    if (not uf.is_same_set(u, v)) {
      uf.unite(u, v);
      tot += pool[i].first;
      edges[u].push_back(PLI(pool[i].first, v));
      edges[v].push_back(PLI(pool[i].first, u));
      edges_i[u].push_back(PLI(pool[i].first, v));
      edges_i[v].push_back(PLI(pool[i].first, u));
    }
  }
  LowestCommonAncestor lca(n, edges_i);
  REP(i, 0, q) {
    int s,t;
    cin >> s >> t;
    s--, t--;
    // max edge in s-t path
    ll tk = lca.lca(s, t);
    cout << tot - tk << endl;
  }
}
