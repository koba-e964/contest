#include <algorithm>
#include <iostream>
#include <queue>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

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
    const Len inf = 1e8;
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


using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;



int main(void){
  int n, v, ox, oy;
  cin >> n >> v >> ox >> oy;
  vector<VI> l(n, VI(n));
  REP(i, 0, n) {
    REP(j, 0, n) {
      cin >> l[i][j];
    }
  }
  // Constructs a graph
  Dijkstra<int> dijk(n * n);
  REP(i, 0, n) {
    REP(j, 0, n) {
      int dx[4] = {1, 0, -1, 0};
      int dy[4] = {0, 1, 0, -1};
      REP(d, 0, 4) {
	int tx = i + dx[d];
	int ty = j + dy[d];
	if (0 > tx || tx >= n || 0 > ty || ty >= n) {
	  continue;
	}
	dijk.add_edge(tx * n + ty, i * n + j, l[i][j]);
      }
    }
  }
  VI sol = dijk.solve(0);
  bool reachable = false;
  if (sol[n * n - 1] < v) {
    reachable = true;
  }
  if (not reachable && ox >= 1) {
    ox--, oy--;
    int to_oasis = sol[oy * n + ox];
    if (to_oasis < v) {
      int rem = 2 * (v - to_oasis);
      VI sol2 = dijk.solve(oy * n + ox);
      reachable = sol2[n * n - 1] < rem;
    }
  }
  cout << (reachable ? "YES": "NO") << endl;
}
