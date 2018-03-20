#include <cassert>
#include <cstring>
#include <string>
#include <iostream>
#include <algorithm>
#include <queue>
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
    const Len inf = 1e8;
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


using namespace std;
const int N=301;

int dp[4][N][N];

// 0: "", 1: "i", 2: "w", 3: "iw"

int tbl[4][4] = {
  {0, 1, 2, 3},
  {1, -1, 3, -1},
  {2, -1, -1, -1},
  {3, 0, -1, -1},
};

int main(void){
  string s;
  cin >> s;
  int n = s.length();
  memset(dp, 0, sizeof(dp));
  REP(i, 0, n) {
    dp[0][i][i] = 1;
  }
  REP(i, 0, n) {
    if (s[i] == 'i') {
      dp[1][i][i + 1] = 1;
    } else {
      dp[2][i][i + 1] = 1;
    }
  }
  REP(s, 1, n + 1) {
    REP(i, 0, n - s + 1) {
      int j = i + s;
      REP(x, 0, 4) {
	REP(y, 0, 4) {
	  int targ = tbl[x][y];
	  if (targ == -1) continue;
	  REP(k, i, j) {
	    if (dp[x][i][k] && dp[y][k][j]) {
	      dp[targ][i][j] = 1;
	      break;
	    }
	  }
	}
      }
    }
  }
  Dijkstra<int> dijk(n + 1);
  REP(i, 0, n) {
    dijk.add_edge(i, i + 1, 1);
  }
  REP(i, 0, n) {
    REP(j, i + 1, n + 1) {
      if (dp[0][i][j]) {
	dijk.add_edge(i, j, 0);
      }
    }
  }
  int dis = dijk.solve(0)[n];
  assert ((n - dis) % 3 == 0);
  cout << (n - dis) / 3 << endl;
}
