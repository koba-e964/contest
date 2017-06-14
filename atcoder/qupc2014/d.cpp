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
typedef pair<int, ll> PIL;

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

ll calc(const VL &x, const VL &g, ll d) {
  int idx = upper_bound(x.begin(), x.end(), d) - x.begin() - 1;
  return g[idx];
}

int main(void){
  int n, m, k, s, g;
  cin >> n >> m >> k >> s >> g;
  Dijkstra<ll> dijk(n);
  REP(i, 0, m) {
    int a, b;
    ll d;
    cin >> a >> b >> d;
    dijk.add_edge(a, b, d);
    dijk.add_edge(b, a, d);
  }
  VL x(k), f(k);
  REP(i, 0, k) {
    cin >> x[i] >> f[i];
  }
  VL sol_s = dijk.solve(s);
  VL sol_g = dijk.solve(g);
  ll mi = 1e16;
  REP(i, 0, n) {
    ll ds = sol_s[i];
    ll dg = sol_g[i];
    mi = min(mi, calc(x, f, ds) + calc(x, f, dg));
  }
  mi = min(mi, calc(x, f, sol_s[g]));
  cout << mi << endl;
}
