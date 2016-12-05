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


const int N = 100100;
VI edges[N];
int depth[N];
int par[N];
int tbl[N];
int n, m;

int DEBUG = 0;
const int B = 18;

int dist(const vector<VI> &spt, const VI &dep, int u, int v) {
  // lca
  int old_u = u;
  int old_v = v;
  int tot = dep[u] + dep[v];
  int du = dep[u];
  int dv = dep[v];
  if (du > dv) {
    swap(du, dv);
    swap(u, v);
  }
  for (int b = B - 1; b >= 0; --b) {
    if (du + (1 << b) <= dv) {
      v = spt[b][v];
      dv -= 1 << b;
    }
  }
  assert (du == dv);
  int lca = -1;
  if (u == v) {
    lca = u;
  } else {
    for (int b = B - 1; b >= 0; --b) {
      if (spt[b][u] != spt[b][v]) {
	v = spt[b][v];
	dv -= 1 << b;
	u = spt[b][u];
	du -= 1 << b;
      }
    }
    lca = spt[0][u];
  }
  int ret = tot - 2 * dep[lca];
  if (DEBUG) { cerr << "dist(" << old_u << "," << old_v << ")=" << ret << endl; }
  return ret;
}

// enumerates all bridges
void dfs1(int v, int p = -1, int d = 0) {
  par[v] = p;
  depth[v] = d;
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    if (p == w) { continue; }
    if (par[w] != -2) { // backlink
      if (depth[w] < d) {
	tbl[w]++;
	tbl[v]--;
      }
    } else {
      dfs1(w, v, d + 1);
    }
  }
}

void dfs2(int v) {
  int tmp = tbl[v];
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    if (par[w] != v) { continue; }
    dfs2(w);
    tmp += tbl[w];
  }
  tbl[v] = tmp;
}

void dfs3(const vector<VI> &trees, VI &par, VI &dep, int v, int p = -1, int d = 0) {
  par[v] = p;
  dep[v] = d;
  REP(i, 0, trees[v].size()) {
    int w = trees[v][i];
    if (DEBUG) {
      cerr << "v = " << v << " -> w = " << w << endl;
    }
    if (w == p) { continue; }
    assert (dep[w] == -1);
    dfs3(trees, par, dep, w, v, d + 1);
  }
}

vector<PI> bridges(void) {
  dfs1(0, -1);
  // imos, cumulative sum
  dfs2(0);
  if (DEBUG) {
    REP(i, 0, n) {
      cerr << "par[" << i << "]=" << par[i] << ", tbl[i] = " << tbl[i] << endl;
    }
  }
  vector<PI> ret;
  REP(i, 1, n) {
    if (tbl[i] == 0) {
      ret.push_back(PI(par[i], i));
    }
  }
  return ret;
}

int main(void){
  cin >> n >> m;
  REP(i, 0, m) {
    int x, y;
    cin >> x >> y;
    x--, y--;
    edges[x].push_back(y);
    edges[y].push_back(x);
  }
  REP(i, 0, N) {
    par[i] = -2;
  }
  vector<PI> brd = bridges(); //bridges
  if (DEBUG) {
    REP(i, 0, brd.size()) {
      cerr << "br:" << brd[i].first << " " << brd[i].second << endl;
    }
  }
  set<PI> brd_s(brd.begin(), brd.end());
  UnionFind uf(n);
  REP(i, 0, n) {
    REP(j, 0, edges[i].size()) {
      int w = edges[i][j];
      if (brd_s.count(PI(i, w)) || brd_s.count(PI(w, i))) {
	continue;
      }
      uf.unite(i, w);
    }
  }
  // contract edges
  vector<VI> tree(n);
  REP(i, 0, n) {
    REP(j, 0, edges[i].size()) {
      int w = edges[i][j];
      if (i > w || uf.is_same_set(i, w)) {
	continue;
      }
      int ri = uf.root(i);
      int rw = uf.root(w);
      tree[ri].push_back(rw);
      tree[rw].push_back(ri);
    }
  }
  // lca
  VI par(n);
  VI dep(n, -1);
  dfs3(tree, par, dep, uf.root(0));
  const int B = 20;
  vector<VI> spt(B, VI(n));
  REP(i, 0, n) {
    spt[0][i] = par[i];
  }
  REP(b, 1, B) {
    REP(i, 0, n) {
      int tmp = spt[b - 1][i];
      spt[b][i] = tmp < 0 ? -1 : spt[b - 1][tmp];
    }
  }
  if (DEBUG) {
    REP(b, 0, B) {
      cerr << "spt[" << b << "]:";
      REP(i, 0, n) {
	cerr << spt[b][i] << " ";
      }
      cerr << endl;
    }
  }
  
  int q;
  cin >> q;
  REP(loopcnt, 0, q) {
    int a, b, c;
    cin >> a >> b >> c;
    a--, b--, c--;
    a = uf.root(a);
    b = uf.root(b);
    c = uf.root(c);
    int x = dist(spt, dep, a, b);
    int y = dist(spt, dep, c, b);
    int z = dist(spt, dep, a, c);
    if (x == 0 || y == 0 || x + y == z) {
      cout << "OK" << endl;
    } else {
      cout << "NG" << endl;
    }
  }
}
