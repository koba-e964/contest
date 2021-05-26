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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

typedef pair<ll, int> PLI;




const int N = 123456;
int n;
vector<PLI> edges[N];


const ll inf = 1e12;

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


ll dfs(int v, int p, ll ma) {
  ll tot = 0;
  if (p != -1) {
    tot += ma;
  }
  REP(i, 0, edges[v].size()) {
    PLI cw = edges[v][i];
    ll c = cw.first;
    int w = cw.second;
    if (w == p) continue;
    tot += dfs(w, v, min(ma, c));
  }
  return tot;
}

void naive(void) {
  VL ans(n);
  REP(i, 0, n) {
    ans[i] = dfs(i, -1, inf);
  }
  REP(i, 0, n) {
    cout << ans[i] << "\n";
  }
  return;
}

void canto(void) {
  UnionFind uf(n);
  REP(v, 0, n) {
    REP(i, 0, edges[v].size()) {
      PLI cw = edges[v][i];
      ll c = cw.first;
      int w = cw.second;
      if (c == 2) {
	uf.unite(v, w);
      }
    }
  }
  VI c(n);
  REP(i, 0, n) {
    c[uf.root(i)] += 1;
  }
  REP(i, 0, n) {
    int t = n - 1 + c[uf.root(i)] - 1;
    cout << t << "\n";
  }
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n;
  ll cma = 0;
  REP(i, 0, n - 1) {
    int a, b;
    ll c;
    cin >> a >> b >> c;
    a--, b--;
    edges[a].push_back(PLI(c, b));
    edges[b].push_back(PLI(c, a));
    cma = max(cma, c);
  }
  if (n <= 1000) {
    naive();
    return 0;
  }
  if (cma <= 2) {
    canto();
    return 0;
  }
}
