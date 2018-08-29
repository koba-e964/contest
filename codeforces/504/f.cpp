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
typedef pair<ll, PI> PLPI;

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

const int N = 500100;
VI g[N];


const int B = 19;
int bin[B][N];
bool vis[N];
ll dat[B][N];
map<PI, ll> determ;
const ll inf = 1e18;
int dep[N];
void dfs(int v, int par, int d = 0) {
  vis[v] = true;
  bin[0][v] = par == -1 ? v : par;
  dep[v] = d;
  if (determ.count(PI(v, par))) {
    dat[0][v] = determ[PI(v, par)];
  }
  for (int w: g[v]) {
    if (w == par) continue;
    dfs(w, v, d + 1);
  }
}

int lca(int x, int y) {
  if (dep[x] > dep[y]) swap(x, y);
  for (int b = B - 1; b >= 0; --b) {
    if (dep[y] - dep[x] >= 1 << b) {
      y = bin[b][y];
    }
  }
  assert (dep[x] == dep[y]);
  if (x == y) return x;
  for (int b = B - 1; b >= 0; --b) {
    if (bin[b][x] != bin[b][y]) {
      x = bin[b][x];
      y = bin[b][y];
    }
  }
  assert (bin[0][x] == bin[0][y] && x != y);
  return bin[0][x];
}

void upd(int des, int anc, ll x) {
  int d = dep[des] - dep[anc];
  for (int b = B - 1; b >= 0; --b) {
    if (d >= 1 << b) {
      dat[b][des] = min(dat[b][des], x);
      d -= 1 << b;
      des = bin[b][des];
    }
  }
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k, m;
  cin >> n >> k >> m;
  UnionFind uf(n);
  REP(i, 0, k) {
    int ga, gb;
    cin >> ga >> gb;
    ga--, gb--;
    g[ga].push_back(gb);
    g[gb].push_back(ga);
    uf.unite(ga, gb);
  }
  vector<PLPI> es;
  REP(i, 0, m) {
    int a, b;
    ll w;
    cin >> a >> b >> w;
    a--, b--;
    es.push_back(PLPI(w, PI(a, b)));
  }
  sort(es.begin(), es.end());
  vector<PLPI> es_nxt;
  for (PLPI e: es) {
    ll w = e.first;
    int a = e.second.first, b = e.second.second;
    if (uf.is_same_set(a, b)) {
      es_nxt.push_back(e);
      continue;
    }
    uf.unite(a, b);
    determ[PI(a, b)] = w;
    determ[PI(b, a)] = w;
    g[a].push_back(b);
    g[b].push_back(a);
  }
  REP(i, 0, n) {
    REP(j, 0, B) {
      bin[j][i] = i;
      dat[j][i] = inf;
    }
  }
  dfs(0, -1);
  REP(b, 1, B) {
    REP(i, 0, n) {
      bin[b][i] = bin[b - 1][bin[b - 1][i]];
    }
  }
  ll tot = 0;
  for (PLPI e: es_nxt) {
    ll w = e.first;
    int a = e.second.first, b = e.second.second;
    int l = lca(a, b);
    upd(a, l, w);
    upd(b, l, w);
  }
  for (int b = B - 1; b >= 1; --b) {
    // reduce
    REP(i, 0, n) {
      dat[b - 1][i] = min(dat[b - 1][i], dat[b][i]);
      int u = bin[b - 1][i];
      dat[b - 1][u] = min(dat[b - 1][u], dat[b][i]);
    }
  }
  REP(i, 0, n) {
    if (bin[0][i] == i) continue;
    if (determ.count(PI(i, bin[0][i]))) continue;
    if (dat[0][i] >= inf) {
      cout << -1 << "\n";
      return 0;
    }
    tot += dat[0][i];
  }
  cout << tot << "\n";
}
