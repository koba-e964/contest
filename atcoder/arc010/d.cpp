#include <algorithm>
#include <cassert>
#include <set>
#include <vector>

/**
 * Strong connected components.
 * Header requirement: algorithm, cassert, set, vector
 * Verified by: AtCoder ARC010 (http://arc010.contest.atcoder.jp/submissions/1015294)
 */
class SCC {
private:
  int n;
  int ncc;
  typedef std::vector<int> vi;
  std::vector<vi> g; // graph in adjacent list
  std::vector<vi> rg; // reverse graph
  vi vs;
  std::vector<bool> used;
  vi cmp;
public:
  SCC(int n): n(n), ncc(-1), g(n), rg(n), vs(n), used(n), cmp(n) {}
  void add_edge(int from, int to) {
    g[from].push_back(to);
    rg[to].push_back(from);
  }
private:
  void dfs(int v) {
    used[v] = true;
    for (int i = 0; i < g[v].size(); ++i) {
      if (!used[g[v][i]]) { dfs(g[v][i]); }
    }
    vs.push_back(v);
  }
  void rdfs(int v, int k) {
    used[v] = true;
    cmp[v] = k;
    for (int i = 0; i < rg[v].size(); ++i) {
      if (!used[rg[v][i]]) { rdfs(rg[v][i], k); }
    }
  }
public:
  int scc() {
    std::fill(used.begin(), used.end(), 0);
    vs.clear();
    for (int v = 0; v < n; ++v) {
      if (!used[v]) { dfs(v); }
    }
    std::fill(used.begin(), used.end(), 0);
    int k = 0;
    for (int i = vs.size() - 1; i >= 0; --i) {
      if (!used[vs[i]]) { rdfs(vs[i], k++); }
    }
    return ncc = k;
  }
  std::vector<int> top_order() const {
    if (ncc == -1) assert(0);
    return cmp;
  }
  /*
   * Returns a dag whose vertices are scc's, and whose edges are those of the original graph.
   */
  std::vector<std::vector<int> > dag() const {
    if (ncc == -1) {
      assert(0);
    }
    typedef std::set<int> si;
    std::vector<si> ret(ncc);
    for (int i = 0; i < g.size(); ++i) {
      for (int j = 0; j < g[i].size(); ++j) {
	int to = g[i][j];
	if (cmp[i] != cmp[to]) {
	  assert (cmp[i] < cmp[to]);
	  ret[cmp[i]].insert(cmp[to]);
	}
      }
    }
    std::vector<std::vector<int> > vret(ncc);
    for (int i = 0; i < ncc; ++i) {
      vret[i] = std::vector<int>(ret[i].begin(), ret[i].end());
    }
    return vret;
  }
  std::vector<std::vector<int> > rdag() const {
    if (ncc == -1) {
      assert(0);
    }
    typedef std::set<int> si;
    std::vector<si> ret(ncc);
    for (int i = 0; i < g.size(); ++i) {
      for (int j = 0; j < g[i].size(); ++j) {
	int to = g[i][j];
	if (cmp[i] != cmp[to]) {
	  assert (cmp[i] < cmp[to]);
	  ret[cmp[to]].insert(cmp[i]);
	}
      }
    }
    std::vector<std::vector<int> > vret(ncc);
    for (int i = 0; i < ncc; ++i) {
      vret[i] = std::vector<int>(ret[i].begin(), ret[i].end());
    }
    return vret;
  }
};


#include <iostream>
#include <cmath>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)


using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;



int paint(int v, VI &deg, const vector<VI> &edges) {
  int n = deg.size();
  if (deg[v] < 0) {
    return 0;
  }
  int sum = 1;
  deg[v] = -1;
  REP(j, 0, edges[v].size()) {
    int w = edges[v][j];
    sum += paint(w, deg, edges);
  }
  return sum;
}

int main(void){
  int n, m;
  cin >> n;
  VL fx(n), fy(n);
  REP(i, 0, n) { cin >> fx[i] >> fy[i]; }
  cin >> m;
  VL sx(m), sy(m);
  REP(i, 0, m) { cin >> sx[i] >> sy[i]; }
  VL audible(n, 9e18);
  REP(i, 0, n) {
    REP(j, 0, m) {
      ll dist_sq = pow(fx[i] - sx[j], 2) + pow(fy[i] - sy[j], 2);
      audible[i] = min(audible[i], dist_sq);
    }
  }
  vector<VI> edges(n);
  REP(i, 0, n) {
    REP(j, 0, n) {
      if (i == j) { continue; }
      ll dist_sq = pow(fx[i] - fx[j], 2) + pow(fy[i] - fy[j], 2);
      if (dist_sq < audible[i]) {
	edges[i].push_back(j);
      }
    }
    if (0) {
      cerr << "edges[" << i << "]:";
      REP(j, 0, edges[i].size()) {
	cerr << " " << edges[i][j];
      }
      cerr << endl;
    }
  }
  SCC scc(n);
  REP(i, 0, n) {
    REP(j, 0, edges[i].size()) {
      int w = edges[i][j];
      scc.add_edge(i, w);
    }
  }
  VI deg(n, 0); // in-degree
  REP(i, 0, n) {
    REP(j, 0, edges[i].size()) {
      int w = edges[i][j];
      deg[w]++;
    }
  }
  scc.scc();
  vector<VI> rdag = scc.rdag();
  int cnt = 0;
  REP(i, 0, rdag.size()) {
    if (rdag[i].size() == 0) { cnt++; }
  }
  cout << cnt << endl;
}
