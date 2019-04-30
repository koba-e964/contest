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
typedef pair<PI, int> PPII;
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


const int N = 100100;
vector<PI> edges[N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  ll k;
  cin >> n >> m >> k;
  
  REP(i, 0, m) {
    int a, b, c;
    cin >> a >> b >> c;
    a--, b--;
    edges[c].push_back(PI(a, b));
  }
  int count = n;
  vector<PPII> whole;
  REP(i, 0, N) {
    set<int> v;
    for (PI e: edges[i]) {
      int x = e.first, y = e.second;
      v.insert(x);
      v.insert(y);
    }
    map<int, int> tbl;
    for (int vv: v) {
      tbl[vv] = count;
      whole.push_back(PPII(PI(count, vv), 1));
      count++;
    }
    for (PI e: edges[i]) {
      int x = e.first, y = e.second;
      whole.push_back(PPII(PI(tbl[x], tbl[y]), 0));
    }
  }
  Dijkstra<ll> dijk(count);
  for (PPII ec: whole) {
    int c = ec.second;
    int x = ec.first.first, y = ec.first.second;
    dijk.add_edge(x, y, c);
    dijk.add_edge(y, x, c);
  }
  ll v = dijk.solve(0)[n - 1];
  if (v >= 1e10) {
    cout << -1 << endl;
  } else {
    cout << v / 2 * k << endl;
  }
}
