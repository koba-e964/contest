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

const int N = 123;
vector<PI> edges[N];

vector<PI> get_cycle(int v, int par, set<int> vis) {
  if (vis.count(v)) {
    // detected
    return vector<PI>(1, PI(v, -1));
  }
  vis.insert(v);
  REP(i, 0, edges[v].size()) {
    PI wc = edges[v][i];
    int w = wc.first;
    if (w == par) { continue; }
    vector<PI> res = get_cycle(w, v, vis);
    if (res.size() > 0) {
      if (res.size() >= 2 && res[0].first == res[res.size() - 1].first) {
	return res;
      }
      res.push_back(PI(v, i));
      return res;
    }
  }
  return vector<PI>();
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m, k;
  cin >> n >> m >> k;
  UnionFind uf(n);
  int conn = n;
  vector<PI> cs;
  // Assume graph is a tree
  REP(i, 0, m) {
    int f, t, c;
    cin >> f >> t >> c;
    f--, t--;
    edges[f].push_back(PI(t, c));
    edges[t].push_back(PI(f, c));
    if (not uf.is_same_set(f, t)) {
      conn--;
    }
    uf.unite(f, t);
  }
  vector<PI> cycle;
  REP(i, 0, n) {
    if (cycle.size() > 0) { break; }
    cycle = get_cycle(i, -1, set<int>());
  }
  vector<set<int> > cycle_dir(n);
  VI testa;
  REP(i, 0, cycle.size()) {
    PI t = cycle[i];
    if (t.second == -1) { continue; }
    int v = t.first;
    int w = edges[v][t.second].first;
    int d = edges[v][t.second].second;
    cycle_dir[v].insert(w);
    cycle_dir[w].insert(v);
    testa.push_back(d);
  }
  REP(i, 0, n) {
    REP(j, 0, edges[i].size()) {
      PI wd = edges[i][j];
      int w = wd.first;
      int d = wd.second;
      if (cycle_dir[i].count(w)) { continue; }
      if (i < w) {
	cs.push_back(PI(d, 0));
      }
    }
  }
  sort(testa.begin(), testa.end());
  if (testa.size() >= 1) {
    assert (testa.size() >= 3);
    cs.push_back(PI(testa[0] + testa[1], -1));
  }
  sort(cs.rbegin(), cs.rend());
  int tot = 0;
  REP(i, 0, k - conn) {
    PI c = cs.back();
    cs.pop_back();
    tot += c.first;
    if (c.second == -1) {
      // pushing
      REP(j, 2, testa.size()) {
	cs.push_back(PI(testa[j], 0));
      }
      sort(cs.rbegin(), cs.rend());
    }
  }
  cout << tot << "\n";
}
