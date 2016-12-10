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


int main(void){
  int n, k, l;
  cin >> n >> k >> l;
  UnionFind uf_a(n), uf_b(n);
  REP(i, 0, k) {
    int p ,q;
    cin >> p >> q;
    p--, q--;
    uf_a.unite(p, q);
  }
  REP(i, 0, l) {
    int r, s;
    cin >> r >> s;
    r--, s--;
    uf_b.unite(r, s);
  }
  map<PI, int> pool;
  REP(i, 0, n) {
    int ra = uf_a.root(i);
    int rb = uf_b.root(i);
    if (pool.count(PI(ra, rb)) == 0) {
      pool[PI(ra, rb)] = 0;
    }
    pool[PI(ra, rb)]++;
  }
  REP(i, 0, n) {
    int ra = uf_a.root(i);
    int rb = uf_b.root(i);
    cout << pool[PI(ra, rb)] << (i == n - 1 ? "\n" : " ");
  }
}
