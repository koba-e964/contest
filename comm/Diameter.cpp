/**
 * Manages a tree and calculates the diameter of it.
 * Header requirement: algorithm, vector
 * Verified by: NJPC 2017-E
 *              (http://njpc2017.contest.atcoder.jp/submissions/1089479)
 */
class Diameter {
  int n;
  typedef long long ll;
  typedef std::pair<int, ll> pil;
  std::vector<std::vector<pil> > edges;
  int x, y;
  void dfs(int v, std::vector<ll> &dist, int p = -1, ll d = 0) const {
    dist[v] = d;
    for (int i = 0; i < edges[v].size(); ++i) {
      int w = edges[v][i].first;
      if (w == p) { continue; }
      dfs(w, dist, v, d + edges[v][i].second);
    }
  }
public:
  Diameter(int n) : n(n), edges(n), x(-1), y(-1) {}
  void add_edge(int a, int b, ll c) {
    edges[a].push_back(pil(b, c));
    edges[b].push_back(pil(a, c));
  }
  std::pair<int, int> diameter(void) {
    if (x >= 0) {
      return std::pair<int, int>(x, y);
    }
    std::vector<ll> dist(n);
    dfs(0, dist);
    int maxi = 0;
    for (int i = 1; i < n; ++i) {
      if (dist[maxi] < dist[i]) {
	maxi = i;
      }
    }
    x = maxi;
    dfs(maxi, dist);
    for (int i = 0; i < n; ++i) {
      if (dist[maxi] < dist[i]) {
	maxi = i;
      }
    }
    y = maxi;
    return std::pair<int, int>(x, y);
  }
  std::vector<ll> farthest(void) {
    if (x < 0) {
      diameter();
    }
    std::vector<ll> ret(n, 0), tmp(n);
    /* For every vertex, the farthest point from it is either x or y. */
    dfs(x, tmp);
    for (int i = 0; i < n; ++i) {
      ret[i] = tmp[i];
    }
    dfs(y, tmp);
    for (int i = 0; i < n; ++i) {
      ret[i] = std::max(ret[i], tmp[i]);
    }
    return ret;
  }
};
