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
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

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

const ll inf = 1e18;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  VL c(n);
  REP(i, 0, n) {
    cin >> c[i];
  }
  UnionFind uf(n);
  REP(i, 0, m) {
    int u, v;
    cin >> u >> v;
    u--, v--;
    uf.unite(u, v);
  }
  VL mi(n, inf);
  REP(i, 0, n) {
    int r = uf.root(i);
    mi[r] = min(mi[r], c[i]);
  }
  ll tot = 0;
  REP(i, 0, n) {
    if (uf.root(i) == i) {
      tot += mi[i];
    }
  }
  cout << tot << "\n";
}
