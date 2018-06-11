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

const int N = 200100;
vector<PI> g[N];
ll rem[N];
ll ei1333[N];

bool vis[N];
// How much flow does v need from its parent?
ll dfs(int v) {
  if (vis[v]) return 0;
  vis[v] = true;
  ll r = rem[v];
  REP(i, 0, g[v].size()) {
    PI we = g[v][i];
    int w = we.first;
    int e = we.second;
    ll sub = dfs(w);
    r -= sub;
    if (e > 0) {
      ei1333[e - 1] += sub;
    } else {
      ei1333[-e - 1] -= sub;
    }
  }
  return -r;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL s(n);
  REP(i, 0, n) cin >> s[i];
  int m;
  cin >> m;
  UnionFind uf(n);
  VI x(m), y(m);
  REP(i, 0, m) {
    cin >> x[i] >> y[i];
    x[i]--, y[i]--;
    uf.unite(x[i], y[i]);
    g[x[i]].push_back(PI(y[i], i + 1));
    g[y[i]].push_back(PI(x[i], -(i + 1)));
  }
  VL sum(n);
  REP(i, 0, n) {
    sum[uf.root(i)] += s[i];
  }
  REP(i, 0, n) {
    if (sum[i] != 0) {
      cout << "Impossible\n";
      return 0;
    }
  }
  cout << "Possible\n";
  REP(i, 0, n) rem[i] = -s[i];
  REP(i, 0, n) {
    if (not vis[i]) dfs(i);
  }
  REP(i, 0, m) {
    cout << ei1333[i] << "\n";
  }
}
