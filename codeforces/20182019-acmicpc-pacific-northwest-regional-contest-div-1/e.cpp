#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>
#include <cassert>
#include <queue>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

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
  /* Perform bfs and calculate distance from s */
  void bfs(int s) {
    level.assign(level.size(), -1);
    std::queue<int> que;
    level[s] = 0;
    que.push(s);
    while (! que.empty()) {
      int v = que.front(); que.pop();
      for (int i = 0; i < (int) graph[v].size(); ++i) {
	edge &e = graph[v][i];
	if (e.cap > 0 && level[e.to] == -1) {
	  level[e.to] = level[v] + 1;
	  que.push(e.to);
	}
      }
    }
  }
  /* search augment path by dfs.
     if f == -1, f is treated as infinity. */
  T dfs(int v, int t, T f) {
    if (v == t) {
      return f;
    }
    for (int &i = iter[v]; i < (int) graph[v].size(); ++i) {
      edge &e = graph[v][i];
      if (e.cap > 0 && level[v] < level[e.to]) {
	T newf = f == -1 ? e.cap : std::min(f, e.cap);
	T d = dfs(e.to, t, newf);
	if (d > 0) {
	  e.cap -= d;
	  graph[e.to][e.rev].cap += d;
	  return d;
	}
      }
    }
    return 0;
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
      bfs(s);
      if (level[t] < 0) {
	std::vector<int> ret;
	for (int i = 0; i < graph.size(); ++i) {
	  if (level[i] < 0) {
	    ret.push_back(i);
	  }
	}
	return std::pair<T, std::vector<int> >(flow, ret);
      }
      iter.assign(iter.size(), 0);
      T f;
      while ((f = dfs(s, t, -1)) > 0) {
	flow += f;
      }
    }
  }
};

const int N = 100;
string s[N];
ll cost[N][N];
const ll inf = 1e10;

int main(void) {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  int n, m, c;
  cin >> m >> n >> c;
  VL w(c);
  REP(i, 0, n) cin >> s[i];
  REP(i, 0, c) cin >> w[i];
  Dinic<ll> din(200000);
  int stdi = -1;
  int dx[4] = {0, 1, 0, -1};
  int dy[4] = {1, 0, -1, 0};
  REP(i, 0, n) {
    REP(j, 0, m) {
      cost[i][j] = inf;
      int di = 2 + 2 * (i * m + j);
      if (s[i][j] >= 'a' && s[i][j] <= 'z') {
        int idx = s[i][j] - 'a';
        assert (idx < c);
        cost[i][j] = w[idx];
      }
      din.add_edge(di, di + 1, cost[i][j]);
      if (s[i][j] == 'B') {
        stdi = di;
      }
      if (i == 0 || i == n - 1 || j == 0 || j == m - 1) {
        din.add_edge(di + 1, 1, inf);
      }
      REP(d, 0, 4) {
        int nx = i + dx[d];
        int ny = j + dy[d];
        if (nx < 0 || nx >= n || ny < 0 || ny >= m) continue;
        int ndi = 2 + 2 * (nx * m + ny);
        din.add_edge(di + 1, ndi, inf);
      }
    }
  }
  assert (stdi >= 0);
  din.add_edge(0, stdi, inf);
  ll ans = din.max_flow(0, 1);
  cout << (ans >= inf ? -1 : ans) << endl;
}
