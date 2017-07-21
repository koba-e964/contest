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


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m, a;
  cin >> n >> m >> a;
  vector<string> s(n);
  Dijkstra<ll> dijk(4 * (a + 1) * n * m);
  int st = -1, go = -1;
  REP(i, 0, n) {
    cin >> s[i];
    REP(j, 0, m) {
      if (s[i][j] == 'S') {
	st = i * m + j;
      }
      if (s[i][j] == 'G') {
	go = i * m + j;
      }
    }
  }
  int dx[4] = {1, 0, -1, 0};
  int dy[4] = {0, 1, 0, -1};
  REP(i, 0, n) {
    REP(j, 0, m) {
      int src = i * m + j;
      if (s[i][j] == '#') { continue; }
      REP(d, 0, 4) {
	if (src == st && d == 2) { continue; }
	int nx = i + dx[d];
	int ny = j + dy[d];
	if (nx < 0 || nx >= n || ny < 0 || ny >= m) { continue; }
	int dst = nx * m + ny;
	REP(lv, 0, a + 1) {
	  dijk.add_edge(d * (a + 1) * n * m + lv * n * m + src,
			d * (a + 1) * n * m + lv * n * m + dst,
			0);
	}
      }
    }
  }
  REP(v, 0, n * m) {
    if (v == st) { continue; }
    REP(lv, 0, a + 1) {
      // 0 <-> 1 and 2 <-> 3, duplicate-side
      if (lv < a) {
	REP(x, 0, 4) {
	  dijk.add_edge(x * (a + 1) * n * m + lv * n * m + v,
			(x ^ 1) * (a + 1) * n * m + (lv + 1) * n * m + v,
			0);
	}
      }
      // 0 <-> 3 and 1 <-> 2, cost-side
      REP(x, 0, 4) {
	dijk.add_edge(x * (a + 1) * n * m + lv * n * m + v,
		      (3 - x) * (a + 1) * n * m + lv * n * m + v,
		      1);
      }
    }
  }
  ll mi = 2 * a + 1;
  VL sol = dijk.solve(0 * (a + 1) * n * m + st);
  REP(d, 0, 4) {
    REP(i, 0, a + 1) {
      mi = min(mi, i + sol[d * (a + 1) * n * m + i * n * m + go]);
    }
  }
  cout << (mi == 2 * a + 1 ? -1 : mi) << "\n";
}
