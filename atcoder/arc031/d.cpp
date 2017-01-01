#include <algorithm>
#include <bitset>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <string>
#include <vector>



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
};


#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)




using namespace std;
typedef long long int ll;
typedef vector<int> VI;

// solved after the author read the editorial
int main(void){
  int n, m;
  cin >> n >> m;
  VI s(n);
  REP(i, 0, n) { cin >> s[i]; }
  VI t(m);
  REP(i, 0, m) { cin >> t[i]; }
  vector<bitset<100> > bit(n);
  REP(i, 0, n) {
    int k;
    cin >> k;
    bitset<100> sol;
    REP(j, 0, k) {
      int u;
      cin >> u;
      sol[u - 1] = true;
    }
    bit[i] = sol;
  }
  const ll BIAS = 3000000;
  ll lo = 1, hi = 10000LL * BIAS;
  while (hi - lo > 1) {
    ll mid = (hi + lo) / 2;
    Dinic<ll> din(2 + n + m);
    ll tot = 0;
    REP(i, 0, n) {
      din.add_edge(0, 2 + i, s[i] * BIAS);
      tot += s[i] * BIAS;
      REP(j, 0, m) {
	if (bit[i][j]) {
	  din.add_edge(2 + i, 2 + n + j, 1LL << 40);
	}
      }
    }
    REP(j, 0, m) {
      din.add_edge(2 + n + j, 1, t[j] * mid);
    }
    if (tot == din.max_flow(0, 1)) {
      hi = mid;
    } else {
      lo = mid;
    }
  }
  printf("%.15f\n", (double) lo / BIAS);
}
