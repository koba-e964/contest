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
typedef pair<int, ll> PIL;

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


const int N = 41260;
const int M = 81600;
int a[M], b[M];
ll c[M], t[M];
set<ll> pl[N];
map<PIL, int> dict;

int main(void){
  int n, m;
  cin >> n >> m;
  REP(i, 0, m) {
    cin >> a[i] >> b[i] >> c[i] >> t[i];
    a[i]--, b[i]--;
    pl[a[i]].insert(c[i]);
    pl[b[i]].insert(c[i]);
  }
  pl[0].insert(1);
  int pts = 0;
  VI terminal;
  REP(i, 0, n) {
    VL cols(pl[i].begin(), pl[i].end());
    REP(j, 0, cols.size()) {
      dict[PIL(i, cols[j])] = pts;
      if (i == n - 1) {
	terminal.push_back(pts);
      }
      pts++;
    }
  }
  Dijkstra<ll> dijk(pts);
  REP(i, 0, m) {
    int u = dict[PIL(a[i], c[i])];
    int v = dict[PIL(b[i], c[i])];
    dijk.add_edge(u, v, t[i]);
    dijk.add_edge(v, u, t[i]);
  }
  REP(i, 0, n) {
    VL cols(pl[i].begin(), pl[i].end());
    REP(j, 0, cols.size() - 1) {
      ll cost = cols[j + 1] - cols[j];
      int u = dict[PIL(i, cols[j])];
      int v = dict[PIL(i, cols[j + 1])];
      dijk.add_edge(u, v, cost);
      dijk.add_edge(v, u, cost);
    }
  }
  int st = dict[PIL(0, 1)];
  VL sol = dijk.solve(st);
  ll mi = 1e15;
  REP(i, 0, terminal.size()) {
    int w = terminal[i];
    mi = min(mi, sol[w]);
  }
  cout << mi << endl;
}
