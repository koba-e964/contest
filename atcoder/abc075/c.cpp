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

using namespace std;
typedef pair<int, int> PI;
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
  int n, m;
  cin >> n >> m;
  vector<PI> edges;
  REP(i, 0, m) {
    int u, v;
    cin >> u >> v;
    edges.push_back(PI(u - 1, v - 1));
  }
  int tot = 0;
  REP(i, 0, m) {
    UnionFind uf(n);
    REP(j, 0, m) {
      if (j == i) { continue; }
      uf.unite(edges[j].first, edges[j].second);
    }
    set<int> conn;
    REP(i, 0, n) {
      conn.insert(uf.root(i));
    }
    if (conn.size() > 1) tot += 1;
  }
  cout << tot << endl;
}
