#include <algorithm>
#include <bitset>
#include <cassert>
#include <deque>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

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
    const Len inf = 1e9;
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
   void solve_with_aid(int source, std::vector<Len> & d, const std::vector<std::pair<Len, int> > &mod) {
     typedef std::pair<Len, int> pi;
     std::priority_queue<pi, std::vector<pi>, std::greater<pi> > que;
     REP(i, 0, mod.size()) {
       que.push(mod[i]);
     }
     while (!que.empty()) {
      pi p = que.top(); que.pop();
      int idx = p.second;
      if (d[idx] <= p.first) {
	continue;
      }
      d[idx] = p.first;
      for(int j = 0; j < edges[idx].size(); ++j) {
	Len newdist = p.first + edges[idx][j].second;
	int nextv = edges[idx][j].first;
	if (d[nextv] > newdist)
	  que.push(pi(newdist, nextv));
      }
    }
  }
};


using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int M = 1410;
int a[M], b[M], c[M];

int main(void){
  int n, m;
  cin >> n >> m;
  REP(i, 0, m) {
    cin >> a[i] >> b[i] >> c[i];
    a[i]--, b[i]--;
  }
  Dijkstra<int> initial(n);
  vector<VI> sols(n);
  REP(i, 0, m) {
    initial.add_edge(a[i], b[i], c[i]);
    initial.add_edge(b[i], a[i], c[i]);
  }
  REP(i, 0, n) {
    sols[i] = initial.solve(i);
  }
  int k;
  cin >> k;
  REP(loop_cnt, 0, k) {
    int x, y, z;
    cin >> x >> y >> z;
    x--, y--;
    ll tot = 0;
    initial.add_edge(x, y, z);
    initial.add_edge(y, x, z);
    REP(i, 0, n) {
      vector<pair<int, int> > mod;
      int ndy = sols[i][x] + z;
      int ndx = sols[i][y] + z;
      if (ndy < sols[i][y]) {
	mod.push_back(make_pair(sols[i][x] + z, y));
      }
      if (ndx < sols[i][x]) {
	mod.push_back(make_pair(sols[i][y] + z, x));
      }
      if (mod.size() > 0) {
	initial.solve_with_aid(i, sols[i], mod);
      }
      REP(j, i + 1, n) {
	tot += sols[i][j];
      }
    }
    cout << tot << endl;
  }
}
