#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <list>
#include <map>
#include <unordered_map>
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
typedef pair<ll, ll> PL;

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
        ll nd = p.first + edges[idx][j].second;
        int w = edges[idx][j].first;
        if (d[w] <= nd) continue;
	que.push(pi(nd, w));
        d[w] = nd + 1;
      }
    }
    return d;
  }
};


int main(void) {
  int n;
  scanf("%d", &n);
  VL a(8);
  REP(i, 0, 8) scanf("%lld", &a[i]);
  VL x(n), y(n);
  REP(i, 0, n) {
    scanf("%lld%lld", &x[i], &y[i]);
  }
  Dijkstra<ll> dijk(9 * n), dijk_rev(9 * n);
  int dx[8] = {0, 1, 1, 1, 0, -1, -1, -1};
  int dy[8] = {1, 1, 0, -1, -1, -1, 0, 1};
  REP(dir, 0, 4) {
    int nx = dx[(dir + 2) % 8];
    int ny = dy[(dir + 2) % 8];
    unordered_map<ll, vector<PL> > hm;
    REP(i, 0, n) {
      ll a = x[i] * dx[dir] + y[i] * dy[dir];
      ll b = x[i] * nx + y[i] * ny;
      hm[b].push_back(PL(a, i));
    }
    for (auto &e: hm) {
      vector<PL> &v = e.second;
      sort(v.begin(), v.end());
      REP(i, 0, v.size() - 1) {
        int p = v[i].second;
        int q = v[i + 1].second;
        // p -> q
        int newdir = dir + 4;
        dijk.add_edge(9 * p + 1 + dir, 9 * q + 1 + dir, 0);
        dijk.add_edge(9 * q + 1 + newdir, 9 * p + 1 + newdir, 0);
        dijk_rev.add_edge(9 * p + 1 + newdir, 9 * q + 1 + newdir, 0);
        dijk_rev.add_edge(9 * q + 1 + dir, 9 * p + 1 + dir, 0);
      }
    }
  }
  REP(i, 0, n) {
    REP(dir, 0, 8) {
      dijk.add_edge(9 * i, 9 * i + 1 + dir, a[dir]);
      dijk.add_edge(9 * i + 1 + dir, 9 * i, 0);
      dijk_rev.add_edge(9 * i, 9 * i + 1 + dir, 0);
      dijk_rev.add_edge(9 * i + 1 + dir, 9 * i, a[dir]);
    }
  }
  VL dist0 = dijk.solve(0);
  VL dist1 = dijk_rev.solve(0);
  REP(i, 1, n) {
    ll ans = dist0[9 * i] + dist1[9 * i];
    printf("%lld\n", ans >= 1e16 ? -1 : ans);
  }
}
