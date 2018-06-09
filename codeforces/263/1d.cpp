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


using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int K = 100010;
int x[K], y[K], p[K];

struct cons {
  int a, b, c;
  cons(int a, int b, int c): a(a), b(b), c(c) {}
};

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k;
  cin >> n >> k;
  REP(i, 0, k) {
    char c;
    cin >> x[i] >> y[i] >> c;
    x[i]--, y[i]--;
    p[i] = c == 'o';
  }
  vector<cons> pool;
  REP(i, 0, k) {
    pool.push_back(cons(x[i]-y[i], x[i]+y[i], p[i]));
  }
  REP(i, 0, n) {
    pool.push_back(cons(-i-1, i-1, 0)); // (-1, i)
    pool.push_back(cons(i+1, i-1, 0)); // (i, -1)
    pool.push_back(cons(-i+n, i+n, 0)); // (n, i)
    pool.push_back(cons(i-n, i+n, 0)); // (i, n)
  }
  // -n <= i <= n, -1 <= j <= 2*n-1,
  // [a[i]=u] => 2*(i+n)+u, [b[j]=u] => 4*n+2+2*(j+1)+u
  UnionFind uf(8*n+4);
  REP(i, 0, pool.size()) {
    cons c = pool[i];
    if (0) {
      DEBUGP(c.a);
      DEBUGP(c.b);
      DEBUGP(c.c);
    }
    REP(d, 0, 2) {
      uf.unite(2 * (c.a + n) + d, 4 * n + 2 + 2 * (c.b + 1) + (c.c + d) % 2);
    }
  }
  REP(i, 0, 4 * n) {
    if (uf.is_same_set(2 * i, 2 * i + 1)) {
      // inconsistent!
      cout << "0\n";
      return 0;
    }
  }
  set<int> conn;
  REP(i, 0, 8 * n + 4) conn.insert(uf.root(i));
  int sz = conn.size(); // 2*n-1 and 4*n-1
  DEBUGP(sz);
  ll ans = 1;
  REP(i, 0, sz / 2 - 2) ans = 2 * ans % mod;
  cout << ans << "\n";
}
