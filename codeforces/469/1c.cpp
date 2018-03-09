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

const int DEBUG = 0;

/**
 * Strong connected components.
 * Header requirement: algorithm, cassert, set, vector
 * Verified by: AtCoder ARC010 (http://arc010.contest.atcoder.jp/submissions/1015294)
 *              yukicoder No. 19 (http://yukicoder.me/submissions/141513)
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
  std::vector<std::vector<int> > scc_components(void) const {
    if (ncc == -1) assert(0);
    std::vector<std::vector<int> > ret(ncc);
    for (int i = 0; i < n; ++i) {
      ret[cmp[i]].push_back(i);
    }
    return ret;
  }
  /*
   * Returns a dag whose vertices are scc's, and whose edges are those of the original graph, in the adjacent-list format.
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


#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m, h;
  cin >> n >> m >> h;
  VI u(n);
  REP(i, 0, n) {
    cin >> u[i];
  }
  vector<VI> c(m, VI(2));
  vector<PI> cons;
  REP(i, 0, m) {
    REP(j, 0, 2) {
      cin >> c[i][j];
      c[i][j]--;
    }
    int diff = u[c[i][0]] - u[c[i][1]];
    if (diff < 0) diff += h;
    if (diff == 1) {
      cons.push_back(PI(c[i][1], c[i][0]));
    }
    if (diff == h - 1) {
      cons.push_back(PI(c[i][0], c[i][1]));
    }
  }
  if (DEBUG) {
    REP(i, 0, cons.size()) {
      cerr << cons[i].first << " ==> " << cons[i].second << endl;
    }
  }
  SCC scc(n);
  REP(i, 0, cons.size()) {
    scc.add_edge(cons[i].first, cons[i].second);
  }
  int ncc = scc.scc();
  VI top_ord = scc.top_order();
  vector<VI> scc_comp = scc.scc_components();
  vector<VI> dag = scc.dag();

  PI min_stop(1e8, -1);
  REP(i, 0, ncc) {
    if (dag[i].size() == 0) {
      min_stop = min(min_stop, PI(scc_comp[i].size(), i));
    }
  }
  VI ans = scc_comp[min_stop.second];
  cout << ans.size() << endl;
  REP(i, 0, ans.size()) {
    cout << (ans[i] + 1) << (i == (int) ans.size() - 1 ? "\n" : " ");
  }
}
