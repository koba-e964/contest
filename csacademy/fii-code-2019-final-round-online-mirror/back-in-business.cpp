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
typedef pair<int, PI> PIPI;

/*
 * Union-Find tree
 * header requirement: vector
 */
class UnionFind {
private:
  std::vector<int> disj;
  std::vector<int> rank;
public:
  UnionFind(int n) : disj(n), rank(n) {
    for (int i = 0; i < n; ++i) {
      disj[i] = i;
      rank[i] = 0;
    }
  }
  int root(int x) {
    if (disj[x] == x) {
      return x;
    }
    return disj[x] = root(disj[x]);
  }
  void unite(int x, int y) {
    x = root(x);
    y = root(y);
    if (x == y) {
      return;
    }
    if (rank[x] < rank[y]) {
      disj[x] = y;
    } else {
      disj[y] = x;
      if (rank[x] == rank[y]) {
	++rank[x];
      }
    }
  }
  bool is_same_set(int x, int y) {
    return root(x) == root(y);
  }
};

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  vector<string> s(n);
  REP(i, 0, n) cin >> s[i];
  int ff = -1, ss = -1;
  const int inf = 1e8;
  vector<VI> dist(n, VI(m, inf));
  queue<PIPI> que;
  REP(i, 0, n) {
    REP(j, 0, m) {
      if (s[i][j] == 'P') {
        que.push(PIPI(0, PI(i, j)));
      }
      if (s[i][j] == 'F') ff = i * m + j;
      if (s[i][j] == 'S') ss = i * m + j;
    }
  }
  int dx[4] = {1, 0, -1, 0};
  int dy[4] = {0, -1, 0, 1};
  while (not que.empty()) {
    PIPI dt = que.front(); que.pop();
    int d = dt.first;
    PI t = dt.second;
    int x = t.first, y = t.second;
    if (dist[x][y] <= d) continue;
    dist[x][y] = d;
    REP(dir, 0, 4) {
      int nx = x + dx[dir];
      int ny = y + dy[dir];
      if (nx < 0 || nx >= n || ny < 0 || ny >= m) continue;
      que.push(PIPI(d + 1, PI(nx, ny)));
    }
  }
  UnionFind uf(n * m);
  vector<PIPI> edges;
  REP(i, 0, n) {
    REP(j, 0, m) {
      if (j > 0) {
        edges.push_back(PIPI(min(dist[i][j - 1], dist[i][j]),
                             PI(i * m + j - 1, i * m + j)));
      }
      if (i > 0) {
        edges.push_back(PIPI(min(dist[i - 1][j], dist[i][j]),
                             PI(i * m + j - m, i * m + j)));
      }
    }
  }
  sort(edges.rbegin(), edges.rend());
  int conn = 0;
  for (auto &e: edges) {
    int c = e.first;
    int x = e.second.first, y = e.second.second;
    uf.unite(x, y);
    if (uf.is_same_set(ff, ss)) {
      conn = c;
      break;
    }
  }
  if (conn == 0) cout << "impossible\n";
  else cout << conn << "\n";
}
