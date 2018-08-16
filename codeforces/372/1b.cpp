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

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<int, ll> PIL;

const int N = 1000;
vector<PIL> g[N];

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

PI srt(PI x) {
  return x.first > x.second ? PI(x.second, x.first) : x;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll base = 1e10;
  int n, m;
  ll l;
  int s, t;
  cin >> n >> m >> l >> s >> t;
  Dijkstra<ll> dijk(n);
  Dijkstra<ll> dijk_mi(n);
  REP(i, 0, m) {
    int u, v;
    ll w;
    cin >> u >> v >> w;
    ll wm = w == 0 ? 1 : w;
    w = w == 0 ? base : w;
    g[u].push_back(PIL(v, w));
    g[v].push_back(PIL(u, w));
    dijk.add_edge(u, v, w);
    dijk.add_edge(v, u, w);
    dijk_mi.add_edge(u, v, wm);
    dijk_mi.add_edge(v, u, wm);
  }
  VL dist = dijk.solve(s);
  VL dist_mi = dijk_mi.solve(s);
  ll d = dist[t];
  if (DEBUG) {
    DEBUGP(d);
    REP(i, 0, n) cerr << " " << dist[i];
    cerr << endl;
    REP(i, 0, n) cerr << " " << dist_mi[i];
    cerr << endl;
  }
  if (d < l || l < dist_mi[t]) {
    cout << "NO\n";
    return 0;
  }
  int cur = t;
  ll rem = l;
  map<PI, ll> costs;
  while (rem > 0) {
    int nxt = -1;
    ll ecost = -1;
    REP(j, 0, g[cur].size()) {
      int w = g[cur][j].first;
      ll c = g[cur][j].second;
      if (c == base) {
	c = rem - dist_mi[w];
      }
      if (c < 1) continue;
      if (dist_mi[w] <= rem - c && rem - c <= dist[w]) {
	nxt = w;
	ecost = c;
	break;
      }
    }
    assert (nxt >= 0);
    costs[srt(PI(nxt, cur))] = ecost;
    cur = nxt;
    rem -= ecost;
  }
  cout << "YES\n";
  REP(i, 0, n) {
    REP(j, 0, g[i].size()) {
      int to = g[i][j].first;
      ll c = g[i][j].second;
      if (i > to) continue;
      if (costs.count(PI(i, to))) c = costs[PI(i, to)];
      cout << i << " " << to << " " << c << "\n";
    }
  }
}
