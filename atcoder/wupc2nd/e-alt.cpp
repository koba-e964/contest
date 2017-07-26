#include <algorithm>
#include <cassert>
#include <iostream>
#include <queue>
#include <set>
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
int n, m, k;

vector<VI> get_cycle_bfs(void) {
  // add all vertices of degree 1
  queue<int> degone;
  vector<vector<int> > ecp(n, VI(n, -1));
  VI deg(n);
  REP(i, 0, n) {
    REP(j, 0, edges[i].size()) {
      ecp[i][edges[i][j].first] = j;
      deg[i] += 1;
    }
  }
  REP(i, 0, n) {
    if (deg[i] == 1) {
      degone.push(i);
    }
  }
  while (not degone.empty()) {
    int t = degone.front(); degone.pop();
    int u = -1;
    REP(i, 0, n) {
      if (ecp[t][i] >= 0) {
	u = i;
      }
    }
    if (u < 0) { continue; }
    ecp[t][u] = -1;
    ecp[u][t] = -1;
    deg[t] -= 1;
    deg[u] -= 1;
    if (deg[u] == 1) {
      degone.push(u);
    }
  }
  // There should remain a single cycle
  return ecp;
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
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
  vector<VI> cycle = get_cycle_bfs();
  vector<set<int> > cycle_dir(n);
  VI testa;
  REP(i, 0, n) {
    REP(j, i + 1, n) {
      if (cycle[i][j] < 0) { continue; }
      int d = edges[i][cycle[i][j]].second;
      cycle_dir[i].insert(j);
      cycle_dir[j].insert(i);
      testa.push_back(d);
    }
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
