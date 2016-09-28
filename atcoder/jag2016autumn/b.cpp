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

const int H = 201;
string s[H];

int main(void){
  int h, w;
  cin >> h >> w;
  REP(i, 0, h) {
    cin >> s[i];
  }
  Dijkstra<ll> dijk(h * w);
  int px = -1, py = -1;
  int gx = -1, gy = -1;

  REP(i, 0, h) {
    REP(j, 0, w) {
      if (s[i][j] == '@') {
	px = i, py = j;
      }
      if (s[i][j] == '%') {
	gx = i, gy = j;
      }
      int dir[5] = {1, 0, -1, 0, 1};
      REP(d, 0, 4) {
	int nx = i + dir[d];
	int ny = j + dir[d + 1];
	if (nx < 0 || nx >= h || ny < 0 || ny >= w) {
	  continue;
	}
	if (s[i][j] == '#' || s[nx][ny] == '#') {
	  continue;
	}
	dijk.add_edge(i * w + j, nx * w + ny, 1);
      }
    }
  }
  VL sol = dijk.solve(gx * w + gy);
  ll dist = sol[px * w + py];
  bool ok = 1;
  REP(i, 0, h) {
    REP(j, 0, w) {
      if (s[i][j] == '$') {
	if (sol[i * w + j] <= dist) {
	  ok = 0;
	}
      }
    }
  }
  cout << (ok ? "Yes" : "No") << endl;
}
