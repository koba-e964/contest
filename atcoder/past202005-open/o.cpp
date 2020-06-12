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

/*
 * Requirement of headers: vector, queue
 * Verified by: POJ 2135 (http://poj.org/problem?id=2135)
 */
class MinCostFlow {
private:
  struct edge {
    int to, rev; // rev is the position of reverse edge in graph[to]
    ll cap, cost;
  };
  typedef std::pair<ll, int> P;
  int v; // the number of vertices
  std::vector<std::vector<edge> > graph;
  std::vector<ll> h; // potential
  std::vector<ll> dist; // minimum distance
  std::vector<int> prevv, preve; // previous vertex and edge
public:
  /* Initializes this solver. v is the number of vertices. */
  MinCostFlow(int v) : v(v), graph(v), h(v), dist(v), prevv(v), preve(v) {}
  /* Initializes this solver with a existing instance. Only graph is copied. */
  MinCostFlow(const MinCostFlow &ano) : v(ano.v), graph(), h(ano.v), dist(ano.v), prevv(ano.v), preve(ano.v) {
    for (int i = 0; i < ano.v; ++i) {
      std::vector<edge> tt;
      for (int j = 0; j < (int) ano.graph[i].size(); ++j) {
	tt.push_back(ano.graph[i][j]);
      }
      graph.push_back(tt);
    }
  }
  /* Adds an edge. */
  void add_edge(int from, int to, ll cap, ll cost) {
    graph[from].push_back((edge) {to, graph[to].size(), cap, cost});
    graph[to].push_back((edge) {from, graph[from].size() - 1, 0, -cost});
  }
  /* Calcucates the minimum cost flow whose source is s, sink is t, and flow is f. */
  ll min_cost_flow(int s, int t, ll f) {
    const ll inf = 1LL << 58;
    ll res = 0;
    std::fill(h.begin(), h.end(), 0);
    while (f > 0) {
      std::priority_queue<P, std::vector<P>, std::greater<P> > que;
      std::fill(dist.begin(), dist.end(), inf);
      dist[s] = 0;
      que.push(P(0, s));
      while (! que.empty()) {
	P p(que.top()); que.pop();
	int v = p.second;
	if (dist[v] < p.first) {
	  continue;
	}
	for (int i = 0; i < (int) graph[v].size(); ++i) {
	  edge &e = graph[v][i];
	  if (e.cap > 0 && dist[e.to] > dist[v] + e.cost + h[v] - h[e.to]) {
	    dist[e.to] = dist[v] + e.cost + h[v] - h[e.to];
	    prevv[e.to] = v;
	    preve[e.to] = i;
	    que.push(P(dist[e.to], e.to));
	  }
	}
      }
      if (dist[t] == inf) {
	return -1; // Cannot add flow anymore
      }
      for (int i = 0; i < v; ++i) {
	h[i] += dist[i];
      }
      // Add flow fully
      ll d = f;
      for (int i = t; i != s; i = prevv[i]) {
	d = std::min(d, graph[prevv[i]][preve[i]].cap);
      }
      f -= d;
      res += d * h[t];
      for (int i = t; i != s; i = prevv[i]) {
	edge &e = graph[prevv[i]][preve[i]];
	e.cap -= d;
	graph[i][e.rev].cap += d;
      }
    } // while (f > 0)
    return res;
  }
};

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  VL a(n), b(n), r(3);
  REP(i, 0, n) cin >> a[i];
  REP(i, 0, n) cin >> b[i];
  REP(i, 0, 3) cin >> r[i];
  MinCostFlow mcf(1 + 3 + n + 1);
  REP(i, 0, 3) {
    mcf.add_edge(0, 1 + i, m, 0);
  }
  const ll BIAS = 1e6;
  REP(i, 0, 3) {
    REP(j, 0, n) {
      ll pro = a[j] * b[j];
      REP(_, 0, i) {
          pro *= b[j];
        }
      pro %= r[i];
      mcf.add_edge(1 + i, 1 + 3 + j, 1, BIAS - pro);
    }
  }
  REP(j, 0, n) {
    VL cos(4);
    REP(i, 0, 3) {
      cos[i + 1] = a[j] * b[j];
      REP(_, 0, i) {
        cos[i + 1] *= b[j];
      }
    }
    REP(i, 0, 3) {
      mcf.add_edge(1 + 3 + j, 1 + 3 + n, 1, cos[i + 1] - cos[i]);
    }
  }
  ll res = mcf.min_cost_flow(0, 1 + 3 + n, 3 * m);
  cout << 3 * m * BIAS - res << endl;
}
