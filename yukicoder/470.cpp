#include <cassert>
#include <iostream>
#include <set>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

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

/**
 * 2-SAT solver.
 * n: the number of variables (v_1, ..., v_n)
 * cons: constraints, given in 2-cnf
 * i (1 <= i <= n) means v_i, -i (1 <= i <= n) means not v_i.
 * Returns: an empty vector if there's no assignment that satisfies cons.
 * Otherwise, it returns an assignment that safisfies cons. (1: true, 0: false)
 */
std::vector<int> two_sat(int n, const vector<pair<int, int> > &cons) {
  SCC scc(2 * n);
  for (int i = 0; i < cons.size(); ++i) {
    pair<int, int> c = cons[i];
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

int main(void){
  int n;
  cin >> n;
  vector<string> u(n);
  REP(i, 0, n) {
    cin >> u[i];
  }
  {
    set<char> res;
    REP(i, 0, n) {
      res.insert(u[i][0]);
      res.insert(u[i][2]);
      if (res.size() < i + 1) {
	cout << "Impossible" << endl;
	return 0;
      }
    }
  }
  assert (n <= 52);
  vector<pair<pair<char, string>, int> > pool;
  REP(i, 0, n) {
    pool.push_back(make_pair(make_pair(u[i][0], u[i].substr(1, 2)), i + 1));
    pool.push_back(make_pair(make_pair(u[i][2], u[i].substr(0, 2)), - (i + 1)));
  }
  vector<PI> interfere;
  REP(i, 0, 2 * n) {
    REP(j, 0, 2 * n) {
      if (i == j) { continue; }
      pair<char, string> t1 = pool[i].first;
      pair<char, string> t2 = pool[j].first;
      if (t1.first == t2.first || t1.second == t2.second) {
	interfere.push_back(make_pair(-pool[i].second, -pool[j].second));
      }
    }
  }
  VI res = two_sat(n, interfere);
  if (res.size() == 0) {
    cout << "Impossible" << endl;
    return 0;
  }
  REP(i, 0, n) {
    if (res[i]) {
      cout << u[i][0] << " " <<  u[i].substr(1, 2) << endl;
    } else {
      cout << u[i].substr(0, 2) << " " << u[i][2] << endl;
    }
  }
}
