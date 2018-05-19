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


#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

int solve(const vector<VI> &a) {
  int n = a.size();
  vector<VI> b(a);
  Dinic<ll> din(5 * n * n + 2);
  REP(i, 0, n) {
    REP(j, 0, n) {
      if (a[i][j] < 0) {
	b[i][j] = -a[i][j] + n - 1;
      } else {
	b[i][j] = a[i][j] - 1;
      }
    }
  }
  REP(i, 0, n) {
    REP(j, 0, n) {
      din.add_edge(2 + n * n + 2 * n * i + b[i][j], 2 + i * n + j, 1);
      din.add_edge(2 + i * n + j, 2 + 3 * n * n + 2 * n * j + b[i][j], 1);
    }
  }
  REP(i, 0, 2 * n * n) {
    din.add_edge(0, 2 + n * n + i, 1);
    din.add_edge(2 + 3 * n * n + i, 1, 1);
  }
  return n * n - din.max_flow(0, 1);
}

int solve_naive(const vector<VI> &a) {
  int n = a.size();
  assert (n <= 4);
  int mi = 1e8;
  REP(bits, 0, 1 << (n * n)) {
    vector<VI> b(a);
    REP(i, 0, n * n) {
      if (bits & 1 << i) {
	int x = i / n;
	int y = i % n;
	b[x][y] = 0;
      }
    }
    bool ok = true;
    REP(i, 0, n) {
      if (not ok) break;
      REP(j, 0, n) {
	if (not ok) break;
	if (b[i][j] == 0) continue;
	REP(k, 0, j) {
	  if (b[i][j] == b[i][k]) {
	    ok = false;
	    break;
	  }
	}
      }
    }
    REP(i, 0, n) {
      if (not ok) break;
      REP(j, 0, n) {
	if (not ok) break;
	if (b[j][i] == 0) continue;
	REP(k, 0, j) {
	  if (b[j][i] == b[k][i]) {
	    ok = false;
	    break;
	  }
	}
      }
    }
    if (ok) {
      mi = min(mi, __builtin_popcount(bits));
    }
  }
  return mi;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  REP(case_nr, 1, t + 1) {
    int n;
    cin >> n;
    vector<VI> a(n, VI(n));
    REP(i, 0, n) REP(j, 0, n) cin >> a[i][j];
    int ans = solve(a);
    cout << "Case #" << case_nr << ": " << ans << endl;
  }
}
