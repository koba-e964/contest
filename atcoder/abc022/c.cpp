#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>
#include <queue>

/*
 * Requirement of headers: vector, queue
 */
class MinCostFlow {
private:
  struct edge {
    int to, cap, cost, rev; // rev is the position of reverse edge in graph[to]
  };
  typedef std::pair<int, int> P;
  int v; // the number of vertices
  std::vector<std::vector<edge> > graph;
  std::vector<int> h; // potential
  std::vector<int> dist; // minimum distance
  std::vector<int> prevv, preve; // previous vertex and edge
public:
  /* Initializes this solver. v is the number of vertices. */
  MinCostFlow(int v) : v(v), graph(v), h(v), dist(v), prevv(v), preve(v) {}
  /* Initializes this solver with a existing instance. Only graph is copied. */
  MinCostFlow(const MinCostFlow &ano) : v(ano.v), graph(), h(ano.v), dist(ano.v), prevv(ano.v), preve(ano.v) {
    for (int i = 0; i < ano.v; ++i) {
      std::vector<edge> tt;
      for (int j = 0; j < ano.graph[i].size(); ++j) {
	tt.push_back(ano.graph[i][j]);
      }
      graph.push_back(tt);
    }
  }
  /* Adds an edge. */
  void add_edge(int from, int to, int cap, int cost) {
    graph[from].push_back((edge) {to, cap, cost, graph[to].size()});
    graph[to].push_back((edge) {from, 0, -cost, graph[from].size() - 1});
  }
  /* Calcucates the minimum cost flow whose source is s, sink is t, and flow is f. */
  int min_cost_flow(int s, int t, int f) {
    const int inf = 0x3fffffff;
    int res = 0;
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
	for (int i = 0; i < graph[v].size(); ++i) {
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
      int d = f;
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
#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;


const int N = 310;
int dist[N][N];

int d0[N];

int main(void){
  int n, m;
  const int inf = 0x3ffffff;
  cin >> n >> m;
  REP(i,0,n) {
    REP(j,0,n) {
      dist[i][j] = inf;
    }
    dist[i][i] = 0;
    d0[i] = inf;
  }
  REP(i,0,m) {
    int u, v, l;
    cin >> u >> v >> l;
    u--, v--;
    dist[u][v] = min(dist[u][v], l);
    dist[v][u] = min(dist[v][u], l);
  }
  REP(i,0,n) {
    d0[i] = dist[0][i];
  }
  REP(k,1,n) {
    REP(i,1,n) {
      REP(j,1,n) {
	dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
      }
    }
  }
  int minv = inf;
  REP(i, 1, n) {
    REP(j,1,n) {
      if (i == j) {
	continue;
      }
      int res = dist[i][j] + d0[i] + d0[j];
      minv = min(minv, res);
    }
  }
  cout << (minv == inf ? -1 : minv) << endl;
}
