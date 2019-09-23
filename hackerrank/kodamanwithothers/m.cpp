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

const ll inf = 1e16;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, s, x;
  cin >> n >> s >> x;
  s -= 2001;
  x -= 2001;
  vector<VI> to(n);
  REP(i, 0, n) {
    int c;
    cin >> c;
    REP(j, 0, c) {
      int t;
      cin >> t;
      to[i].push_back(t);
    }
  }
  vector<VI> e(n, VI(7));
  REP(i, 0, n) {
    REP(j, 0, 7) cin >> e[i][j];
  }
  Dijkstra<ll> dijk(7 * n);
  REP(i, 0, n) {
    REP(j, 0, 7) {
      for (int t: to[i]) {
        t -= 2001;
        int nw = (j + e[i][j]) % 7;
        dijk.add_edge(7 * i + j, 7 * t + nw, e[i][j]);
      }
    }
  }
  VL sol = dijk.solve(7 * s);
  ll mi = inf;
  REP(i, 0, 7) {
    mi = min(mi, sol[7 * x + i]);
  }
  cout << (mi >= inf ? 1048596 : mi) << endl;
}
