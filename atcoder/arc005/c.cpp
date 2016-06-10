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
const int H = 510;
const ll big = 1e9;
string s[H];

int main(void){
  int h, w;
  cin >> h >> w;
  REP(i, 0, h) {
    cin >> s[i];
  }
  Dijkstra<ll> dijk(h * w);
  int ss = 0, gg = 0;
  REP(i, 0, h) {
    REP(j, 0, w) {
      switch(s[i][j]) {
      case 's':
	ss = i * w + j;
	break;
      case 'g':
	gg = i * w + j;
	break;
      default:
	break;
      }
      int dxy[5] = {0, 1, 0, -1, 0};
      REP(d, 0, 4) {
	int fx = i + dxy[d];
	int fy = j + dxy[d + 1];
	if (fx < 0 || fx >= h || fy < 0 || fy >= w) {
	  continue;
	}
	dijk.add_edge(fx * w + fy, i * w + j, s[i][j] == '#' ? big : 1LL);
      }
    }
  }
  
  ll dist = dijk.solve(ss)[gg];
  cout << (dist < 3 * big ? "YES" : "NO") << endl;
}
