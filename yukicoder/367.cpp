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
typedef pair<int, int> PI;
const double EPS=1e-9;
const int inf = 0x1ffffff;

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

string s[500];

int main(void){
  int h, w;
  cin >> h >> w;
  REP(i, 0, h) {
    cin >> s[i];
  }
  Dijkstra<int> dijk(2 * h * w);
  int knight[8][2]= {{-2, -1}, {-2, 1}, {-1, -2}, {-1, 2}, {1, -2}, {1,2}, {2,-1}, {2,1}};
  int bishop[4][2] = {{-1,-1}, {-1,1}, {1,-1}, {1,1}};
  REP(i, 0, h) {
    REP(j, 0, w) {
      REP(q, 0, 8) {
	int nx = i + knight[q][0];
	int ny = j + knight[q][1];
	if (0 > nx || nx >= h) continue;
	if (0 > ny || ny >= w) continue;
	int np = w * nx + ny;
	if (s[nx][ny] == 'R') {
	  np += w * h; // another world
	}
	dijk.add_edge(w * i + j, np, 1);
      }

      REP(q, 0, 4) {
	int nx = i + bishop[q][0];
	int ny = j + bishop[q][1];
	if (0 > nx || nx >= h) continue;
	if (0 > ny || ny >= w) continue;
	int np = w * nx + ny + w * h;
	if (s[nx][ny] == 'R') {
	  np -= w * h; // another world
	}
	dijk.add_edge(w * i + j + w * h, np, 1);
      }
    }
  }
  int sx = 0, sy = 0;
  int gx = 0, gy = 0;
  REP(i, 0, h) {
    REP(j, 0, w) {
      if (s[i][j] == 'S') {
	sx = i, sy = j;
      }
      if (s[i][j] == 'G') {
	gx = i, gy = j;
      }
    }
  }
  vector<int> result = dijk.solve(w * sx + sy);
  int g = w * gx + gy;
  int dist = min(result[g], result[g + w * h]);
  cout << (dist == inf ? -1 : dist) << endl;
}
