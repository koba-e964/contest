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
typedef pair<int, ll> PIL;

const int N = 100100;

/**
 * Dijkstra's algorithm.
 * First, call add_edge() to add edges.
 * Second, call solve() to calculate the length of the shortest path from source to each vertex.
 * Header requirement: algorithm, queue, vector
 * Verified by AtCoder ARC026-C (http://arc026.contest.atcoder.jp/submissions/604231)
 */
 template<class Len = int>
class Dijkstra {
private:
  int n;
  std::vector<std::vector<std::pair<int, Len> > > edges;
public:
  /**
   * n: the number of vertices
   */
  Dijkstra(int n) : n(n), edges(n) {}
  /*
   * from: the source of edge to add
   * to: the target of edge to add
   * cost: the cost of edge to add
   */
  void add_edge(int from, int to, Len cost) {
    edges[from].push_back(std::pair<int, Len>(to, cost));
  }
  /*
   * This function returns an array consisting of the distances from vertex source.
   */
  std::vector<Len> solve(int source) {
    const Len inf = 1e16;
    typedef std::pair<Len, int> pi;
    std::vector<Len> d(n, inf);
    std::priority_queue<pi, std::vector<pi>, std::greater<pi> > que;
    que.push(pi(0, source));
    while (!que.empty()) {
      pi p = que.top(); que.pop();
      int idx = p.second;
      if (d[idx] <= p.first) {
	continue;
      }
      d[idx] = p.first;
      for(int j = 0; j < edges[idx].size(); ++j) {
	que.push(pi(p.first + edges[idx][j].second, edges[idx][j].first));
      }
    }
    return d;
  }
};


vector<PIL> gy[N];
vector<PIL> gs[N];


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m, s, t;
  cin >> n >> m >> s >> t;
  s--, t--;
  Dijkstra<ll> dij_y(n), dij_s(n);
  REP(i, 0, m) {
    int u, v;
    ll a, b;
    cin >> u >> v >> a >> b;
    u--, v--;
    gy[u].push_back(PIL(v, a));
    gy[v].push_back(PIL(u, a));
    gs[u].push_back(PIL(v, b));
    gs[v].push_back(PIL(u, b));
    dij_y.add_edge(u, v, a);
    dij_y.add_edge(v, u, a);
    dij_s.add_edge(u, v, b);
    dij_s.add_edge(v, u, b);
  }
  VL ans(n);
  VL sol_y = dij_y.solve(s);
  VL sol_s = dij_s.solve(t);
  REP(i, 0, n) {
    ans[i] = (ll)1e15 - sol_y[i] - sol_s[i];
  }
  for (int i = n - 2; i >= 0; --i) {
    ans[i] = max(ans[i], ans[i + 1]);
  }
  REP(i, 0, n) {
    cout << ans[i] << "\n";
  }
}
