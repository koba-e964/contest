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
#include <unordered_map>
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
	Len next_cost = p.first + edges[idx][j].second;
	int next_vert = edges[idx][j].first;
	if (d[next_vert] <= next_cost) {
	  continue;
	}
	d[next_vert] = next_cost + 1;
	que.push(pi(next_cost, next_vert));
      }
    }
    return d;
  }
};


const int N = 10000;

VI edge_a[N], edge_b[N];

int llim[N], ulim[N];
int offset[N];
ll all_dist[1500000];


vector<PI> memo_inv;


int main(void){
  cin.tie(0);
  ios::sync_with_stdio(false);
  memo_inv.reserve(1500000);
  int n, m;
  cin >> n >> m;
  Dijkstra<ll> dijk_b(n);
  REP(i, 0, m) {
    int c, a, b;
    cin >> c >> a >> b;
    if (c == 0) {
      edge_a[a].push_back(b);
      edge_a[b].push_back(a);
      dijk_b.add_edge(a, b, 1);
      dijk_b.add_edge(b, a, 1);
    } else {
      edge_b[a].push_back(b);
      edge_b[b].push_back(a);
      dijk_b.add_edge(a, b, 100000);
      dijk_b.add_edge(b, a, 100000);
    }
  }
  VL dist_b = dijk_b.solve(0);
  REP(i, 0, n) {
    ll dist = dist_b[i];
    int num_b = dist / 100000;
    int num_a = dist % 100000;
    int lim_dist = sqrt(ll(num_b + 1) * num_b + 2 * num_a);
    llim[i] = num_b;
    ulim[i] = lim_dist;
    offset[i] = memo_inv.size();
    REP(k, num_b, lim_dist + 1) {
      memo_inv.push_back(PI(k, i));
    }
  }
  assert (memo_inv.size() <= 1500000);
  fill_n(all_dist, 1500000, 1e16);
  typedef std::pair<ll, int> pi;
  std::priority_queue<pi, std::vector<pi>, std::greater<pi> > que;
  que.push(pi(0, 0));
  while (not que.empty()) {
    PI p = que.top(); que.pop();
    if (all_dist[p.second] <= p.first) {
      continue;
    }
    all_dist[p.second] = p.first;
    PI cur = memo_inv[p.second];
    int v = cur.second;
    int used_b = cur.first;
    for (auto dest: edge_a[v]) {
      if (used_b <= ulim[dest]) {
	int id = offset[dest] + used_b - llim[dest];
	if (all_dist[id] <= p.first + 1) { continue; }
	all_dist[id] = p.first + 2;
	que.push(pi(p.first + 1, id));
      }
    }
    for (auto dest: edge_b[v]) {
      if (used_b + 1 <= ulim[dest]) {
	int id = offset[dest] + used_b + 1 - llim[dest];
	if (all_dist[id] <= p.first + 1 + used_b) { continue; }
	all_dist[id] = p.first + 2 + used_b;
	que.push(pi(p.first + 1 + used_b, id));
      }
    }
  }
  VL mi(n, 1e16);
  REP(i, 0, n) {
    REP(j, llim[i], ulim[i] + 1) {
      int idx = offset[i] + j - llim[i];
      ll dist = all_dist[idx];
      mi[i] = min(mi[i], dist);
    }
  }
  REP(i, 0, n) {
    cout << mi[i] << "\n";
  }
}
