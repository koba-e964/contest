#include <vector>
#include <list>
#include <map>
#include <set>
#include <deque>
#include <queue>
#include <stack>
#include <bitset>
#include <algorithm>
#include <functional>
#include <numeric>
#include <utility>
#include <sstream>
#include <iostream>
#include <iomanip>
#include <cstdio>
#include <cmath>
#include <cstdlib>
#include <cctype>
#include <string>
#include <cstring>
#include <ctime>

using namespace std;

typedef long long ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS = 1e-9;
#define rep(i,s,n) for(int i=(s); i < int(n); ++i)
class Dinic {
private:
  struct edge {
    int to, cap, rev; // rev is the position of reverse edge in graph[to]
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
  int dfs(int v, int t, int f) {
    if (v == t) {
      return f;
    }
    for (int &i = iter[v]; i < graph[v].size(); ++i) {
      edge &e = graph[v][i];
      if (e.cap > 0 && level[v] < level[e.to]) {
	int newf = f == -1 ? e.cap : std::min(f, e.cap);
	int d = dfs(e.to, t, newf);
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
  void add_edge(int from, int to, int cap) {
    graph[from].push_back((edge) {to, cap, graph[to].size()});
    graph[to].push_back((edge) {from, 0, graph[from].size() - 1});
  }
  int max_flow(int s, int t) {
    int flow = 0;
    while (1) {
      bfs(s);
      if (level[t] < 0) {
	return flow;
      }
      iter.assign(iter.size(), 0);
      int f;
      while ((f = dfs(s, t, -1)) > 0) {
	flow += f;
      }
    }
  }
};

const int N = 2e6 + 10;
int ca[N], cb[N];
int a[N], b[N], w[N];
int n, m;

int main(void) {
  cin >> n >> m;
  rep(i, 0, n) {
    scanf("%d%d", &ca[i], &cb[i]);
  }
  rep(i, 0, m) {
    scanf("%d%d%d", &a[i], &b[i], &w[i]);
  }
  Dinic din(n + 2); // source : 0, sink : 1
  rep(i, 0, n) {
    din.add_edge(i + 2, 1, ca[i]);
    din.add_edge(0, i + 2, cb[i]);
  }
  rep(i, 0, m) {
    din.add_edge(a[i] + 1, b[i] + 1, w[i]);
    din.add_edge(b[i] + 1, a[i] + 1, w[i]);
  }
  cout << din.max_flow(0, 1) << endl;
}

