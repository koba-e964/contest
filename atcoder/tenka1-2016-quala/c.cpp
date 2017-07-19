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


string dominus() {
  string ret;
  for (char c = 'a'; c <= 'z'; ++c) {
    ret += c;
  }
  return ret;
}

string ecclesia(char a, char b) {
  string ret;
  for (char c = 'a'; c <= 'z'; ++c) {
    if (c == b) { continue; }
    ret += c;
    if (c == a) {
      ret += b;
    }
  }
  return ret;
}

PI solve(const string &a, const string &b) {
  int n = a.length();
  int m = b.length();
  REP(i, 0, min(n, m)) {
    if (a[i] == b[i]) {
      continue;
    }
    return PI(a[i] - 'a', b[i] - 'a');
  }
  if (n < m) {
    return PI(0, 0);
  }
  return PI(-1, -1);
}

void fail(void) {
  cout << -1 << "\n";
  exit(0);
}

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

const int A = 26;

int dist[A][A];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  SCC scc(A);
  REP(i, 0, A) {
    REP(j, 0, A) {
      dist[i][j] = 1;
    }
  }
  REP(loop_cnt, 0, n) {
    string a, b;
    cin >> a >> b;
    PI cons = solve(a, b);
    if (cons.first == -1) {
      fail();
    }
    if (cons.first == 0 && cons.second == 0) {
      continue;
    }
    scc.add_edge(cons.first, cons.second);
    dist[cons.first][cons.second] = 0;
  }
  int ncc = scc.scc();
  if (ncc < 26) {
    fail();
  }
  // Warshall-Floyd
  REP(k, 0, A) {
    REP(i, 0, A) {
      REP(j, 0, A) {
	dist[i][j] = min(dist[i][j], max(dist[i][k], dist[k][j]));
      }
    }
  }
  string ret;
  set<int> unused;
  REP(i, 0, A) {
    unused.insert(i);
  }
  while (not unused.empty()) {
    REP(i, 0, A) {
      if (unused.count(i) == 0) {
	continue;
      }
      bool ok = true;
      REP(j, 0, A) {
	if (dist[j][i] == 0 && unused.count(j)) {
	  ok = false;
	  break;
	}
      }
      if (not ok) {
	continue;
      }
      // ok
      ret += (char)('a' + i);
      unused.erase(i);
      break;
    }
  }
  cout << ret << "\n";
}
