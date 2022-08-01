/**
 * Dinic's algorithm for maximum flow problem.
 * Header requirement: vector, deque
 * Verified by: ABC010-D(http://abc010.contest.atcoder.jp/submissions/602810)
 *              ARC031-D(http://arc031.contest.atcoder.jp/submissions/1050071)
 *              POJ 3155(http://poj.org/problem?id=3155)
 */
template<class T = int>
class Dinic {
private:
  struct edge {
    int to;
    T cap;
    int rev; // rev is the position of reverse edge in graph[to]
  };
  std::vector<std::vector<edge> > graph;
  std::vector<int> level;
  std::vector<int> iter;
  std::deque<int> que;
  /* Perform bfs and calculate distance from s */
  void bfs(int s, int t) {
    level.assign(level.size(), -1);
    level[s] = 0;
    que.clear();
    que.push_back(s);
    while (! que.empty()) {
      int v = que.front(); que.pop_front();
      for (int i = 0; i < (int) graph[v].size(); ++i) {
        edge &e = graph[v][i];
        if (e.cap > 0 && level[e.to] == -1) {
          level[e.to] = level[v] + 1;
          if (e.to == t) return;
          que.push_back(e.to);
        }
      }
    }
  }
  /* search augment path by dfs.
     if f == -1, f is treated as infinity. */
  T dfs(int v, int s, T f) {
    if (v == s) {
      return f;
    }
    T res = 0;
    for (int &i = iter[v]; i < (int) graph[v].size(); ++i) {
      edge &e = graph[v][i];
      T cap = graph[e.to][e.rev].cap;
      if (cap > 0 && level[e.to] >= 0 && level[v] > level[e.to]) {
        T newf = f == -1 ? cap : std::min(f - res, cap);
        T d = dfs(e.to, s, newf);
        if (d > 0) {
          e.cap += d;
          graph[e.to][e.rev].cap -= d;
          res += d;
          if (res == f) return res;
        }
      }
    }
    level[v] = -1;
    return res;
  }
public:
  /* v is the number of vertices (labeled from 0 .. v-1) */
  Dinic(int v) : graph(v), level(v, -1), iter(v, 0) {}
  void add_edge(int from, int to, T cap) {
    if (from == to) return;
    graph[from].push_back((edge) {to, cap, (int) graph[to].size()});
    graph[to].push_back((edge) {from, 0, (int) graph[from].size() - 1});
  }
  T max_flow(int s, int t) {
    T flow = 0;
    while (1) {
      bfs(s);
      if (level[t] < 0) {
        return flow;
      }
      iter.assign(iter.size(), 0);
      T f;
      while ((f = dfs(s, t, -1)) > 0) {
        flow += f;
      }
    }
  }
  std::pair<T,std::vector<int> > max_flow_cut(int s, int t) {
    T flow = 0;
    while (1) {
      bfs(s, t);
      if (level[t] < 0) {
        std::vector<int> ret;
        for (int i = 0; i < (int) graph.size(); ++i) {
          if (level[i] < 0) {
            ret.push_back(i);
          }
        }
        return std::pair<T, std::vector<int> >(flow, ret);
      }
      iter.assign(iter.size(), 0);
      T f = dfs(t, s, -1);
      flow += f;
    }
  }
};
