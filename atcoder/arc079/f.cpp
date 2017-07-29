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

const int DEBUG = 0;


using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;


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


/**
 * 2-SAT solver.
 * n: the number of variables (v_1, ..., v_n)
 * cons: constraints, given in 2-cnf
 * i (1 <= i <= n) means v_i, -i (1 <= i <= n) means not v_i.
 * Returns: an empty vector if there's no assignment that satisfies cons.
 * Otherwise, it returns an assignment that safisfies cons. (1: true, 0: false)
 * Dependencies: SCC.cpp
 * Verified by: Codeforces #400 D
 *              (http://codeforces.com/contest/776/submission/24942283)
 */
std::vector<int> two_sat(int n, const vector<pair<int, int> > &cons) {
  SCC scc(2 * n);
  for (int i = 0; i < cons.size(); ++i) {
    std::pair<int, int> c = cons[i];
    int x, y;
    if (c.first > 0) {
      x = c.first - 1 + n;
    } else {
      x = -c.first - 1;
    }
    if (c.second > 0) {
      y = c.second - 1;
    } else {
      y = -c.second - 1 + n;
    }
    scc.add_edge(x, y);
    scc.add_edge((y + n) % (2 * n), (x + n) % (2 * n));
  }
  scc.scc();
  std::vector<int> result(n);
  std::vector<int> top_ord = scc.top_order();
  REP(i, 0, n) {
    if (top_ord[i] == top_ord[i + n]) {
      return std::vector<int>();
    }
    result[i] = top_ord[i] > top_ord[i + n] ? 1 : 0;
  }
  return result;
}



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI p(n);
  SCC scc(n);
  vector<VI> edges(n);
  REP(i, 0, n) {
    cin >> p[i];
    p[i]--;
    scc.add_edge(p[i], i);
    edges[p[i]].push_back(i);
  }
  int ncc = scc.scc();
  vector<VI> comp = scc.scc_components();
  vector<VI> dag = scc.dag();
  vector<VI> rdag = scc.rdag();
  VI dp(ncc);
  for (int i = ncc - 1; i >= 0; --i) {
    set<int> mex;
    if (rdag[i].size() == 0) {
      // terminal. cycle
      dp[i] = -1;
      continue;
    }
    REP(j, 0, dag[i].size()) {
      int w = dag[i][j];
      // mex
      mex.insert(dp[w]);
    }
    int m = 0;
    while (mex.count(m)) {
      m += 1;
    }
    dp[i] = m;
  }
  VI ndp(n, -1);
  VI top_ord = scc.top_order();
  REP(i, 0, n) {
    ndp[i] = dp[top_ord[i]];
  }
  bool ok = true;
  REP(i, 0, comp.size()) {
    if (comp[i].size() >= 2) {
      int cs = comp[i].size();
      if (DEBUG) {
	cerr << "Cycle";
	REP(j, 0, cs) {
	  cerr << " "  << comp[i][j];
	}
	cerr << endl;
      }
      /* Check */
      vector<VI> poss(cs, VI(2, -1));
      REP(j, 0, cs) {
	int v = comp[i][j];
	// mex dp[j] s.t. v -> j
	set<int> used;
	REP(k, 0, edges[v].size()) {
	  int w = edges[v][k];
	  used.insert(ndp[w]);
	}
	VI tas(2);
	int pos = 0;
	int c = 0;
	while (pos < 2) {
	  if (used.count(c)) {
	    c += 1;
	  } else {
	    tas[pos] = c;
	    c += 1;
	    pos++;
	  }
	}
	poss[j] = tas;
      }
      // Check integrity
      map<int, int> inv;
      REP(j, 0, cs) {
	inv[comp[i][j]] = j;
      }
      vector<PI> cons;
      REP(j, 0, cs) {
	int w = comp[i][j];
	int v = p[w];
	assert (inv.count(v) != 0);
	int iv = inv[v];
	// iv -> j in comp[i]
	// x[j] != poss[iv][0] ===> x[iv] == poss[iv][0]
	// x[j] == poss[iv][0] ===> x[iv] == poss[iv][1]
	// variable: if x[k] == poss[k][1] true else false
	int tmp = poss[iv][0];
	if (poss[j][0] == tmp) {
	  cons.push_back(PI((j + 1), iv + 1));
	  cons.push_back(PI(-(j + 1), -(iv + 1)));
	} else if (poss[j][1] == tmp) {
	  cons.push_back(PI(-(j + 1), iv + 1));
	  cons.push_back(PI((j + 1), -(iv + 1)));
	} else {
	  cons.push_back(PI(-(iv + 1), -(iv + 1)));
	}
      }
      if (DEBUG) {
	cerr << "constraints:" << endl;
	REP(i, 0, cons.size()) {
	  cerr << -cons[i].first << "  ====> " << cons[i].second << endl;
	}
	cerr << endl;
      }
      VI sat = two_sat(cs, cons);
      if (sat.size() == 0) {
	ok = false;
	break;
      }
    }
  }
  cout << (ok ? "POSSIBLE" : "IMPOSSIBLE") << "\n";
}
