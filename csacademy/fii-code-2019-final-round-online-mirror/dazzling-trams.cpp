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
const ll mod = 1e9 + 7;


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
    const Len inf = 1e8;
    typedef std::pair<Len, int> pi;
    std::vector<Len> d(n, inf);
    std::deque<pi> que;
    que.push_back(pi(0, source));
    while (!que.empty()) {
      pi p = que.front(); que.pop_front();
      int idx = p.second;
      if (d[idx] <= p.first) {
	continue;
      }
      d[idx] = p.first;
      for(int j = 0; j < edges[idx].size(); ++j) {
        if (edges[idx][j].second == 1) {
          que.push_back(pi(p.first + 1, edges[idx][j].first));
        } else {
          que.push_front(pi(p.first, edges[idx][j].first));
        }
      }
    }
    return d;
  }
};

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m, k;
  cin >> n >> m >> k;
  int u = 1;
  while ((1 << u) - 1 <= k) u++;
  u--;
  DEBUGP(k);
  DEBUGP(u);
  vector<VI> routes(m);
  int count = n;
  vector<PI> ne;
  vector<PI> oe;
  REP(i, 0, m) {
    int len;
    cin >> len;
    routes[i] = VI(len);
    REP(j, 0, len) {
      cin >> routes[i][j];
      routes[i][j]--;
    }
    int fst = count;
    int diff = k - (1 << u) + 1;
    REP(j, 0, len) {
      oe.push_back(PI(routes[i][j], fst + j));
      if (j + diff < len) {
        oe.push_back(PI(routes[i][j], fst + j + diff));
      }
      if (j >= diff) {
        oe.push_back(PI(routes[i][j], fst + j - diff));
      }
    }
    REP(l, 0, u) {
      int diff = 1 << l;
      REP(j, 0, len) {
        int nxt = l == u - 1 ? routes[i][j] : fst + (l + 1) * len + j;
        ne.push_back(PI(fst + l * len + j, nxt));
        if (j + diff < len) {
          nxt = l == u - 1 ? routes[i][j + diff] : fst + (l + 1) * len + j + diff;
          ne.push_back(PI(fst + l * len + j, nxt));
        }
        if (j >= diff) {
          nxt = l == u - 1 ? routes[i][j - diff] : fst + (l + 1) * len + j - diff;
          ne.push_back(PI(fst + l * len + j, nxt));
        }
      }
    }
    count += len * u;
  }
  Dijkstra<int> dijk(count);
  for (auto e: ne) {
    dijk.add_edge(e.first, e.second, 0);
  }
  for (auto e: oe) {
    dijk.add_edge(e.first, e.second, 1);
  }
  VI a = dijk.solve(0);
  cout << a[n - 1] << "\n";
}
