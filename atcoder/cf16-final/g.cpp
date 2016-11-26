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

typedef pair<ll, PI> PLPI;

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
  int n, q;
  cin >> n >> q;
  VI a(q), b(q);
  VL c(q);
  REP(i, 0, q) {
    cin >> a[i] >> b[i] >> c[i];
  }
  VL perim(n, 1e17);
  REP(i, 0, q) {
    perim[a[i]] = min(perim[a[i]], c[i] + 1);
    perim[b[i]] = min(perim[b[i]], c[i] + 2);
  }

  REP(i, 0, 2 * n) {
    perim[(i + 1) % n] = min(perim[(i + 1) % n], perim[i % n] + 2);
  }
  
  UnionFind uf(n);
  vector<PLPI> pool;
  ll tot = 0;
  REP(i, 0, q) {
    pool.push_back(PLPI(c[i], PI(a[i], b[i])));
  }
  REP(i, 0, n) {
    pool.push_back(PLPI(perim[i], PI(i, (i + 1) % n)));
  }
  sort(pool.begin(), pool.end());
  REP(i, 0, pool.size()) {
    int u = pool[i].second.first;
    int v = pool[i].second.second;
    if (not uf.is_same_set(u, v)) {
      uf.unite(u, v);
      tot += pool[i].first;
    }
  }
  cout << tot << endl;
}
