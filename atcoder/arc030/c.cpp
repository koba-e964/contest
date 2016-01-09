

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

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;


string dp[400][400];

int main(void){
  int n, m, k;
  cin >> n >> m >> k;
  vector<char> c(n);
  SCC scc(n);
  REP(i, 0, n) cin >> c[i];
  REP(i, 0, m) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    scc.add_edge(a, b);
  }
  int ncc = scc.scc();
  VI top_ord = scc.top_order();
  vector<VI> rdag = scc.rdag();
  vector<string> pool(ncc);
  REP(i, 0, n) {
    pool[top_ord[i]] += c[i];
  }
  REP(i, 0, ncc) {
    sort(pool[i].begin(), pool[i].end());
  }
  REP(i, 0, ncc) {
    REP(j, 0, k + 1) {
      dp[i][j] = "\x7e"; // infinity
    }
  }
  REP(i, 0, ncc) { // dp[i]
    REP(j, 0, pool[i].length() + 1) {
      dp[i][j] = pool[i].substr(0, j);
    }
    REP(j, 0, pool[i].length() + 1) {
      REP(t, 0, rdag[i].size()) {
	int prev = rdag[i][t];
	REP(u, 0, k - j + 1) {
	  dp[i][u + j] = min(dp[i][u + j], dp[prev][u] + pool[i].substr(0, j));
	}
      }
    }
  }
  string ret = "\x7e";
  REP(i, 0, ncc) {
    ret = min(ret, dp[i][k]);
  }
  cout << (ret == "\x7e" ? "-1" : ret) << endl;

}
