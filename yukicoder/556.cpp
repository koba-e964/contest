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
  UnionFind uf(n);
  VI boss(n);
  VI num(n);
  REP(i, 0, n) {
    boss[i] = i;
    num[i] = 1;
  }
  REP(i, 0, m) {
    int x, y;
    cin >> x >> y;
    x--, y--;
    int rx = uf.root(x);
    int ry = uf.root(y);
    int nx = num[rx];
    int ny = num[ry];
    if (rx == ry) {
      continue;
    }
    if (PI(-nx, rx) > PI(-ny, ry)) {
      swap(rx, ry);
      swap(nx, ny);
    }
    int bossx = boss[rx];
    uf.unite(rx, ry);
    boss[uf.root(rx)] = bossx;
    num[uf.root(rx)] = nx + ny;
  }
  REP(i, 0, n) {
    cout << boss[uf.root(i)] + 1 << endl;
  }
}
