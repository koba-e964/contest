#include <cassert>
#include <deque>
#include <iomanip>
#include <iostream>
#include <string>
#include <vector>

/**
 * Dinic's algorithm for maximum flow problem.
 * Header requirement: vector, queue
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

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef vector<int> VI;

const int inf = 1e8;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  while (t--) {
    int n, m;
    cin >> n >> m;
    vector<string> s(n);
    REP(i, 0, n) cin >> s[i];
    REP(i, 0, n) {
      REP(j, 0, m) {
        if ((i + j) % 2 == 0) continue;
        if (s[i][j] == 'W') s[i][j] = 'B';
        else if (s[i][j] == 'B') s[i][j] = 'W';
      }
    }
    Dinic<int> din(2 + n * m + 2 * (n - 1) * (m - 1));
    REP(i, 0, n) {
      REP(j, 0, m) {
        int x = i * m + j;
        if (s[i][j] == 'W')
          din.add_edge(0, 2 + x, inf);
        if (s[i][j] == 'B')
          din.add_edge(2 + x, 1, inf);
      }
    }
    REP(i, 0, n - 1) {
      REP(j, 0, m - 1) {
        int x = i * (m - 1) + j;
        din.add_edge(0, 2 + n * m + x, 1);
        din.add_edge(2 + n * m + (n - 1) * (m - 1) + x, 1, 1);
        REP(a, 0, 2) {
          REP(b, 0, 2) {
            int y = 2 + (i + a) * m + (j + b);
            din.add_edge(2 + n * m + x, y, 1);
            din.add_edge(y, 2 + n * m + (n - 1) * (m - 1) + x, 1);
          }
        }
      }
    }
    pair<int, VI> macut = din.max_flow_cut(0, 1);
    int ma = macut.first;
    VI cut = macut.second;
    cout << 2 * (n - 1) * (m - 1) - ma << "\n";
    vector<string> ans(n, string(m, 'W'));
    for (int v: cut) {
      if (2 <= v && v < 2 + n * m) {
        int x = (v - 2) / m;
        int y = (v - 2) % m;
        ans[x][y] = 'B';
      }
    }
    REP(i, 0, n) {
      REP(j, 0, m) {
        if ((i + j) % 2 == 0) continue;
        if (ans[i][j] == 'W') ans[i][j] = 'B';
        else if (ans[i][j] == 'B') ans[i][j] = 'W';
      }
    }
    REP(i, 0, n) cout << ans[i] << "\n";
  }
}
