#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
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

int main() {
  int n;
  cin >> n;
  set<PI> e;
  VI used;
  REP(i, 0, n) {
    int x[3];
    REP(j, 0, 3) cin >> x[j];
    int prod = x[0] * x[1] * x[2];
    REP(j, 0, 3) {
      REP(k, 1, x[j]) {
        int p = prod / x[j] * k;
        int q = prod - p;
        if (p > q) {
          swap(p, q);
        }
        e.insert(PI(p, q));
        used.push_back(p);
        used.push_back(q);
      }
    }
  }
  sort(used.begin(), used.end());
  used.erase(unique(used.begin(), used.end()), used.end());
  int m = used.size();
  int k = e.size();
  if (m + k == 0) {
    cout << 0 << endl;
    return 0;
  }
  Dinic<ll> din(2 * m + 2);
  REP(i, 0, m) {
    din.add_edge(0, 2 + i, 1);
    din.add_edge(2 + m + i, 1, 1);
  }
  auto it = e.begin();
  REP(i, 0, k) {
    PI f = *(it++);
    int x = lower_bound(used.begin(), used.end(), f.first) - used.begin();
    int y = lower_bound(used.begin(), used.end(), f.second) - used.begin();
    assert(f.first == used[x]);
    assert(f.second == used[y]);
    din.add_edge(2 + x, 2 + m + y, 1);
    if (x != y) {
      din.add_edge(2 + y, 2 + m + x, 1);
    }
  }
  cout << 2 * m - din.max_flow(0, 1) << endl;
}
