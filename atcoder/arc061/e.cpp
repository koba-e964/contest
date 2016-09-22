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

map<PI, int> reg;
vector<PI> reg_v;
vector<vector<PI> > edges;
int regi(PI p) {
  if (reg.count(p)) {
    return reg[p];
  }
  int s = reg_v.size();
  reg_v.push_back(p);
  edges.push_back(vector<PI>());
  reg[p] = s;
  return s;
}

int main(void){
  int n, m;
  cin >> n >> m;
  REP(i, 0, m) {
    int p, q, c;
    cin >> p >> q >> c;
    p--, q--, c--;
    int idx1 = regi(PI(p, c));
    int idx2 = regi(PI(q, c));
    edges[idx1].push_back(PI(idx2, 0));
    edges[idx2].push_back(PI(idx1, 0));
    int bot1 = regi(PI(p, -1));
    int bot2 = regi(PI(q, -1));
    edges[bot1].push_back(PI(idx2, 1));
    edges[idx1].push_back(PI(bot1, 0));
    edges[bot2].push_back(PI(idx1, 1));
    edges[idx2].push_back(PI(bot2, 0));
  }
  int st = regi(PI(0, -1));
  int en = regi(PI(n - 1, -1));
  Dijkstra<ll> dijk(reg_v.size());
  REP(i, 0, reg_v.size()) {
    REP(j, 0, edges[i].size()) {
      PI p = edges[i][j];
      dijk.add_edge(i, p.first, p.second);
    }
  }
  ll res = dijk.solve(st)[en];
  cout << (res == 1e16 ? -1 : res) << endl;
}
