/**
 * Bipartite Matching with a simple dfs.
 * Header requirement: vector
 * Verified by: POJ 1274 (http://poj.org/problem?id=1742)
 */
class BipartiteMatching {
private:
  std::vector<std::vector<int> > graph; // [v1][?]
  std::vector<int> match; // [v2]
  std::vector<bool> used; // [v1]
public:
  /* v is the number of vertices (labeled from 0 .. v-1) */
  BipartiteMatching(int v1, int v2) : graph(v1), match(v2), used(v1) {}
  void add_edge(int from, int to) {
    graph[from].push_back(to);
  }
  bool dfs(int v) {
    used[v] = true;
    for (int i = 0; i < graph[v].size(); ++i) {
      int u = graph[v][i];
      int w = match[u];
      if (w < 0 || (!used[w] && dfs(w))) {
	match[u] = v;
	return true;
      }
    }
    return false;
  }
  int matching(void) {
    int res = 0;
    std::fill(match.begin(), match.end(), -1);
    int n = match.size();
    for (int v = 0; v < graph.size(); ++v) {
      std::fill(used.begin(), used.end(), false);
      if (dfs(v)) {
	res++;
      }
    }
    return res;
  }
};
