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
