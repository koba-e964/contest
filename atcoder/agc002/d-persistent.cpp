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

/*
 * Persistent Union Find tree.
 * Reference: https://misteer.hatenablog.com/entry/persistentUF
 * Header requirement: vector, utility
 * Verified by https://beta.atcoder.jp/contests/agc002/submissions/3185458
 */
class PersistentUnionFind {
public:
  static const int inf = 1e8;
  typedef std::pair<int, int> pii;
  std::vector<int> par, time, rank;
  std::vector<std::vector<pii> > num; // [(time, size of this component)]
  int now;
  PersistentUnionFind(int n):
    par(n), time(n, inf), rank(n, 0), num(n), now(0) {
    for (int i = 0; i < n; ++i) {
      par[i] = i;
      num[i].push_back(pii(-1, 1));
    }
  }
  int root(int x, int t) const {
    while (1) {
      if (time[x] > t) return x;
      x = par[x];
    }
  }
  // returns the current time
  int unite(int x, int y) {
    x = this->root(x, now);
    y = this->root(y, now);
    if (x == y) return now;
    ++now;
    if (rank[x] <= rank[y]) swap(x, y);
    par[y] = x;
    time[y] = now;
    rank[x] = max(rank[x], rank[y] + 1);
    num[x].push_back(pii(now, num[x].back().second + num[y].back().second));
    return now;
  }
  int size(int x, int t) const {
    x = root(x, t);
    return (upper_bound(num[x].begin(), num[x].end(), pii(t, inf)) - 1)->second;
  }
};
const int PersistentUnionFind::inf;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  VI time(m);
  PersistentUnionFind uf(n);
  REP(i, 0, m) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    time[i] = uf.unite(a, b);
  }
  int q;
  cin >> q;
  REP(i, 0, q) {
    int x, y, z;
    cin >> x >> y >> z;
    x--, y--;
    int pass = m, fail = 0;
    while (pass - fail > 1) {
      int mid = (pass + fail) / 2;
      int t = time[mid - 1];
      int rx = uf.root(x, t);
      int ry = uf.root(y, t);
      int size = rx == ry ? uf.size(rx, t) : uf.size(rx, t) + uf.size(ry, t);
      if (size >= z) pass = mid;
      else fail = mid;
    }
    cout << pass << endl;
  }
}
