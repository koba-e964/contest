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
   std::vector<Len> solve(int source,int q) {
    const Len inf = 0;
    typedef std::pair<Len, int> pi;
    std::vector<Len> d(n, inf);
    std::priority_queue<pi, std::vector<pi>, std::less<pi> > que;
    que.push(pi(q, source));
    while (!que.empty()) {
      pi p = que.top(); que.pop();
      int idx = p.second;
      if (d[idx] >= p.first) {
	continue;
      }
      d[idx] = p.first;
      for(int j = 0; j < edges[idx].size(); ++j) {
	que.push(pi(min(p.first, edges[idx][j].second), edges[idx][j].first));
      }
    }
    return d;
  }
};
int main(void){
  int n, m, q;
  cin >> n >> m >> q;
  Dijkstra<int> dijk(n);
  set<PI> e;
  REP(i, 0, m) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    e.insert(PI(a, b));
  }
  REP(i, 0, q) {
    int c, d;
    cin >> c >> d;
    c--, d--;
    dijk.add_edge(c, d, i + 1);
    dijk.add_edge(d, c, i + 1);
    e.erase(PI(c, d));
  }
  for (set<PI>::iterator it = e.begin(); it != e.end(); ++it) {
    int a = it->first;
    int b = it->second;
    dijk.add_edge(a, b, q + 1);
    dijk.add_edge(b, a, q + 1);
  }
  VI res = dijk.solve(0, q + 1);
  REP(i, 0, n - 1) {
    int r = res[i + 1] == q + 1 ? -1 : res[i + 1];
    cout << r << endl;
  }
}
