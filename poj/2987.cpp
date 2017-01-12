#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

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
  pair<T, vector<T> > max_flow_cut(int s, int t) {
    T flow = 0;
    while (1) {
      bfs(s);
      if (level[t] < 0) {
	vector<T> cut;
	REP(i, 0, graph.size()) {
	  if (level[i] >= 0) {
	    cut.push_back(i);
	  }
	}
	return pair<T, vector<T> >(flow, cut);
      }
      iter.assign(iter.size(), 0);
      T f;
      while ((f = dfs(s, t, -1)) > 0) {
	flow += f;
      }
    }
  }
};


int main(void){
  int n, m;
  scanf("%d%d", &n, &m);
  Dinic<ll> din(n + 2);
  const ll bias = 1e7;
  const ll inf = 1e12;
  REP(i, 0, n) {
    ll v;
    scanf("%lld", &v);
    din.add_edge(i + 2, 1, (bias - v) * (n + 1) + 1);
    din.add_edge(0, i + 2, bias * (n + 1));
  }
  REP(i, 0, m) {
    int u, v;
    scanf("%d%d", &u, &v);
    u--, v--;
    din.add_edge(u + 2, v + 2, inf);
  }
  ll result = din.max_flow(0, 1);
  ll ans = result / (n + 1) - n * bias;
  assert (ans <= 0);
  printf("%lld %lld\n", result % (n + 1), -ans);
}
