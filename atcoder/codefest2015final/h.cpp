#include <algorithm>
#include <queue>
#include <vector>
/**
 * Dijkstra's algorithm.
 * First, call add_edge() to add edges.
 * Second, call solve() to calculate the length of the shortest path from source to each vertex.
 * Header requirement: algorithm, queue, vector
 * Verified by Code Festival 2015 Final H
 */
class Dijkstra {
private:
  int n;
  std::vector<std::vector<std::pair<int, int> > > edges;
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
  void add_edge(int from, int to, int cost) {
    edges[from].push_back(std::pair<int, int>(to, cost));
  }
  /*
   * This function returns an array consisting of the distances from vertex source.
   */
  std::vector<int> solve(int source) {
    const int inf = 1e8;
    typedef std::pair<int, int> pi;
    std::vector<int> d(n, inf);
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

const int N = 1e5 + 10;

vector<PI> edges[N];

int main(void){
  int n,m;
  cin >> n >> m;
  VI s(n), l(n);
  Dijkstra dijk(m + 1);
  REP(i, 0, n) {
    cin >> s[i] >> l[i];
    dijk.add_edge(s[i], s[i] + l[i], l[i]);
  }
  REP(i, 0, m) {
    dijk.add_edge(i, i + 1, 2);
    dijk.add_edge(i + 1, i, 0);
  }
  cout << 2 * m - dijk.solve(0)[m] << endl;
}
