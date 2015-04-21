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



using namespace std;

typedef long long ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS = 1e-9;
#define rep(i,s,n) for(int i=(s); i < int(n); ++i)

const int M = 1e4 + 10;
int a[M], b[M], w[M];
int n, m;

#include <iostream>
#include <cstdio>

int main(void) {
  cin >> n >> m;
  rep(i, 0, m) {
    scanf("%d%d%d", a + i, b + i, w + i);
    a[i]--;
    b[i]--;
  }
  MinCostFlow mcf(n);
  rep(i, 0, m) {
    mcf.add_edge(a[i], b[i], 1, w[i]);
    mcf.add_edge(b[i], a[i], 1, w[i]);
  }
  cout << mcf.min_cost_flow(0, n - 1, 2) << endl;
}

