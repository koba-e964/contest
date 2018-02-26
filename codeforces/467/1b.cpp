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
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

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
 * Dijkstra's algorithm.
 * First, call add_edge() to add edges.
 * Second, call solve() to calculate the length of the shortest path from source to each vertex.
 * Header requirement: algorithm, queue, vector
 * Verified by AtCoder ARC026-C (http://arc026.contest.atcoder.jp/submissions/604231)
 */
 template<class Len = int>
class Dijkstra {
private:
  int n;
  std::vector<std::vector<std::pair<int, Len> > > edges;
public:
  /**
   * n: the number of vertices
   */
  Dijkstra(int n) : n(n), edges(n) {}
  /*
   * from: the source of edge to add
   * to: the target of edge to add
   * cost: the cost of edge to add
   */
  void add_edge(int from, int to, Len cost) {
    edges[from].push_back(std::pair<int, Len>(to, cost));
  }
  /*
   * This function returns an array consisting of the distances from vertex source.
   */
  std::vector<Len> solve(int source) {
    const Len inf = 1e16;
    typedef std::pair<Len, int> pi;
    std::vector<Len> d(n, inf);
    std::priority_queue<pi, std::vector<pi>, std::greater<pi> > que;
    que.push(pi(0, source));
    while (!que.empty()) {
      pi p = que.top(); que.pop();
      int idx = p.second;
      if (d[idx] <= p.first) {
	continue;
      }
      d[idx] = p.first;
      for(int j = 0; j < edges[idx].size(); ++j) {
	que.push(pi(p.first + edges[idx][j].second, edges[idx][j].first));
      }
    }
    return d;
  }
};



const int N = 100100;
VI edges[N];

vector<VI> dag;
VI top_ord;
VL dist;

int dp[N];
int cc[N];

int dfs_draw(int v) {
  if (dp[v] != -2) {
    return dp[v];
  }
  int ans = -1;
  if (cc[v] >= 2) {
    ans = 0;
    return dp[v] = 0;
  }
  REP(i, 0, dag[v].size()) {
    int w = dag[v][i];
    int sub = dfs_draw(w);
    if (sub == 0) {
      ans = 0;
      break;
    }
  }
  dp[v] = ans;
  return ans;
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  SCC scc(n);
  REP(i, 0, n) {
    int c;
    cin >> c;
    edges[i] = VI(c);
    REP(j, 0, c) {
      cin >> edges[i][j];
      edges[i][j]--;
      scc.add_edge(i, edges[i][j]);
    }
  }
  int s;
  cin >> s;
  s--;
  // is_winning?
  {
    Dijkstra<ll> dijk(2 * n);
    vector<VI> rev(2 * n);
    REP(i, 0, n) {
      REP(j, 0, edges[i].size()) {
	int w = edges[i][j];
	dijk.add_edge(i, n + w, 1);
	dijk.add_edge(n + i, w, 1);
	rev[n + w].push_back(i);
	rev[w].push_back(n + i);
      }
    }
    // Can you reach some vertices with outdegree 0 in an odd number of steps?
    VL dist = dijk.solve(s);
    int dst = -1;
    REP(i, 0, n) {
      if (edges[i].size() != 0) continue;
      if (dist[n + i] <= 2 * n) {
	// ok
	dst = n + i;
	break;
      }
    }
    if (dst != -1) {
      VI ans;
      int rem = dist[dst];
      while (rem > 0) {
	int tmp = -1;
	REP(i, 0, rev[dst].size()) {
	  int w = rev[dst][i];
	  if (dist[w] == rem - 1) {
	    tmp = w;
	    break;
	  }
	}
	assert (tmp >= 0);
	ans.push_back(dst >= n ? dst - n : dst);
	rem -= 1;
	dst = tmp;
      }
      assert (dst == s);
      ans.push_back(s);
      reverse(ans.begin(), ans.end());
      cout << "Win\n";
      REP(i, 0, ans.size()) {
	cout << ans[i] + 1 << (i == (int) ans.size() - 1 ? "\n" : " ");
      }
      return 0;
    }
  }
  int ncc = scc.scc();
  dag = scc.dag();
  top_ord = scc.top_order();
  REP(i, 0, n) cc[top_ord[i]] += 1;
  Dijkstra<ll> dijk(ncc);
  REP(i, 0, ncc) {
    REP(j, 0, dag[i].size()) {
      dijk.add_edge(i, dag[i][j], 1);
    }
  }
  int dags = top_ord[s];
  dist = dijk.solve(dags);
  if (0) {
    cerr << "dag-dist:";
    for (auto d:dist)cerr<<" "<<d;
    cerr << endl;
  }
  REP(i, 0, ncc) dp[i] = -2;
  int draw = dfs_draw(dags);
  if (draw == 0) {
    cout << "Draw\n";
  } else {
    cout << "Lose\n";
  }
}
