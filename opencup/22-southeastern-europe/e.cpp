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

/*
 * Union-Find tree
 * header requirement: vector
 */
class UnionFind {
private:
  std::vector<int> disj;
  std::vector<int> rank;
public:
  UnionFind(int n) : disj(n), rank(n) {
    for (int i = 0; i < n; ++i) {
      disj[i] = i;
      rank[i] = 0;
    }
  }
  int root(int x) {
    if (disj[x] == x) {
      return x;
    }
    return disj[x] = root(disj[x]);
  }
  void unite(int x, int y) {
    x = root(x);
    y = root(y);
    if (x == y) {
      return;
    }
    if (rank[x] < rank[y]) {
      disj[x] = y;
    } else {
      disj[y] = x;
      if (rank[x] == rank[y]) {
	++rank[x];
      }
    }
  }
  bool is_same_set(int x, int y) {
    return root(x) == root(y);
  }
};

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

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  vector<string> a(n), b(n);
  REP(i, 0, n) cin >> a[i];
  REP(i, 0, n) cin >> b[i];
  UnionFind uf(n * m);
  REP(i, 0, n) {
    REP(j, 0, m - 1) {
      int v = i * m + j;
      if (a[i][j] == a[i][j + 1]) {
        uf.unite(v, v + 1);
      }
    }
  }
  REP(i, 0, n - 1) {
    REP(j, 0, m) {
      int v = i * m + j;
      if (a[i][j] == a[i + 1][j]) {
        uf.unite(v, v + m);
      }
    }
  }
  vector<int> rs;
  vector<int> col;
  vector<int> id(n * m);
  REP(i, 0, n * m) {
    if (uf.root(i) == i) {
      id[i] = rs.size();
      rs.push_back(i);
      col.push_back(a[i / m][i % m] - '0');
    }
  }
  int k = rs.size();
  VI c0(k), c1(k);
  REP(i, 0, n * m) {
    int r = uf.root(i);
    int idx = id[r];
    if (b[i / m][i % m] == '0') c1[idx]++;
    else c0[idx]++;
  }
  Dinic<ll> din(2 + k);
  const ll inf = 1e15;
  ll ans = 0;
  REP(i, 0, k) {
    if (c0[i] < c1[i]) {
      ans += c0[i];
      din.add_edge(2 + i, 1, c1[i] - c0[i]);
    } else {
      ans += c1[i];
      din.add_edge(0, 2 + i, c0[i] - c1[i]);
    }
  }
  vector<set<int>> e(k);
  REP(i, 0, n) {
    REP(j, 0, m - 1) {
      int v = i * m + j;
      if (a[i][j] != a[i][j + 1]) {
        int idx1 = id[uf.root(v)];
        int idx2 = id[uf.root(v + 1)];
        if (col[idx1] == 1) swap(idx1, idx2);
        e[idx1].insert(idx2);
      }
    }
  }
  REP(i, 0, n - 1) {
    REP(j, 0, m) {
      int v = i * m + j;
      if (a[i][j] != a[i + 1][j]) {
        int idx1 = id[uf.root(v)];
        int idx2 = id[uf.root(v + m)];
        if (col[idx1] == 1) swap(idx1, idx2);
        e[idx1].insert(idx2);
      }
    }
  }
  REP(i, 0, k) {
    // cerr << "c:" << c0[i] << " " << c1[i] << endl;
    for (auto to: e[i]) {
      // cerr << i << " " << to << endl;
      din.add_edge(2 + i, 2 + to, inf);
    }
  }
  ll ma = din.max_flow(0, 1);
  cout << ans + ma << endl;
}
