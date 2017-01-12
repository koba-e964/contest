#include <vector>
#include <queue>
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
      for (int i = 0; i < graph[v].size(); ++i) {
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
    for (int &i = iter[v]; i < graph[v].size(); ++i) {
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
    graph[from].push_back((edge) {to, cap, graph[to].size()});
    graph[to].push_back((edge) {from, 0, graph[from].size() - 1});
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
  std::pair<T,std::vector<int> >  max_flow_cut(int s, int t) {
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


#include <algorithm>
#include <cmath>
#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;



int main(void){
  ios::sync_with_stdio(false);
  int n, m;
  cin >> n >> m;
  vector<PI> edges;
  VI idx(n, 0);
  REP(i, 0, m) {
    int u, v;
    cin >> u >> v;
    u--;
    v--;
    edges.push_back(PI(u, v));
    idx[u]++;
    idx[v]++;
  }
  double lo = 0;
  double hi = n;
  REP(loop_cnt, 0, 40) {
    double mid = (hi + lo) / 2;
    Dinic<double> din(n + m + 2);
    const double inf = 1e5;
    REP(i, 0, m) {
      PI t = edges[i];
      din.add_edge(i + 2, m + t.first + 2, inf);
      din.add_edge(i + 2, m + t.second + 2, inf);
      din.add_edge(0, i + 2, 1);
    }
    REP(i, 0, n) {
      din.add_edge(0, m + i + 2, mid);
      din.add_edge(m + i + 2, 1, idx[i]);
    }
    double res = 2 * m - din.max_flow(0, 1);
    if (res < 1e-9) {
      hi = mid;
    } else {
      lo = mid;
    }
  }
  // guess a fraction that represents res
  double mindiff = 1e8;
  int num = -1, den = -1;
  REP(b, 1, 100) {
    int q = hi * b + 0.5;
    double diff = abs((double) q / b - hi);
    if (mindiff > diff) {
      mindiff = diff;
      num = q;
      den = b;
    }
  }
  Dinic<ll> din(n + m + 2);
  const ll inf = 1e9;
  REP(i, 0, m) {
    PI t = edges[i];
    din.add_edge(i + 2, m + t.first + 2, inf);
    din.add_edge(i + 2, m + t.second + 2, inf);
    din.add_edge(0, i + 2, den);
  }
  REP(i, 0, n) {
    din.add_edge(0, m + i + 2, num);
    din.add_edge(m + i + 2, 1, idx[i] * den);
  }
  pair<ll, VI> res = din.max_flow_cut(0, 1);
  VI st = res.second;
  VI ans;
  REP(i, 0, st.size()) {
    if (st[i] >= 2 + m) {
      ans.push_back(st[i] - m - 2);
    }
  }

  cout << ans.size() << endl;
  REP(i, 0, ans.size()) {
    cout << ans[i] + 1 << endl;
  }
}
