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

const int N = 112222;
int a[N], b[N], w[N];

typedef pair<ll, PI> PLPI;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m, k;
  cin >> n >> m >> k;
  VI c(n);
  REP(i, 0, n) {
    cin >> c[i];
  }
  vector<PLPI> pool;
  REP(i, 0, m) {
    cin >> a[i] >> b[i] >> w[i];
    a[i]--, b[i]--;
    pool.push_back(PLPI(w[i], PI(a[i], b[i])));
  }
  sort(pool.begin(), pool.end());
  UnionFind uf(k + n);
  int conn = k;
  ll tot = 0;
  REP(i, 0, pool.size()) {
    if (conn <= 1) break;
    ll ww = pool[i].first;
    int x = pool[i].second.first;
    int y = pool[i].second.second;
    x = c[x] == 0 ? k + x : c[x] - 1;
    y = c[y] == 0 ? k + y : c[y] - 1;
    assert (x >= 0);
    assert (y >= 0);
    if (not uf.is_same_set(x, y)) {
      tot += ww;
      uf.unite(x, y);
      conn--;
    }
  }
  cout << (conn > 1 ? -1 : tot) << "\n";
}
