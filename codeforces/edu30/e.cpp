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

/*
 * Sparse Table.
 * BiOp should be the type of a binary operator which is
 * associative, commutative and idempotent.
 * (For example, min and gcd satisfy them.)
 * Header Requirement: vector, cassert
 * Verified by: AtCoder ARC023 D (http://arc023.contest.atcoder.jp/submissions/960757)
 */
template<class T, class BiOp>
class SparseTable {
private:
  BiOp biop;
  std::vector<std::vector<T> > st;
  void create_sparse_table(int n, const std::vector<T> &lcp) {
    int h = 1;
    while ((1 << h) < n) {
      ++h;
    }
    st = std::vector<std::vector<T> >(h + 1, std::vector<T>(n));

    for (int i = 0; i < n; ++i) {
      st[0][i] = lcp[i];
    }
    for (int j = 1; j <= h; ++j) {
      for (int i = 0; i <= n - (1 << j); ++i) {
	st[j][i] = biop(st[j - 1][i], st[j - 1][i + (1 << (j-1))]);
      }
    }
  }
  /*
   * Reference: https://graphics.stanford.edu/~seander/bithacks.html#IntegerLogFloat
   * Please be aware that it only works well in case of 1 <= t <= 2^24.
   */
  static int top_bit(int t) {
    const float v = t; // find int(log2(v)), where v > 0.0 && finite(v) && isnormal(v)
    int c;         // 32-bit int c gets the result;
    
    c = *(const int *) &v;  // OR, for portability:  memcpy(&c, &v, sizeof c);
    return (c >> 23) - 127;
  }
public:
  /*
   * Initializes this sparse table. O(n log n) where n = ary.size().
   */
  SparseTable(BiOp biop, const std::vector<T> &ary): biop(biop) {
    create_sparse_table(ary.size(), ary);
  }
  /*
   * Computes biop(ary[f], ary[f+1], ..., ary[s]). O(1).
   * Note: the interval is inclusive.
   */
  T query(int f, int s) const {
    assert (f <= s);
    int diff = top_bit(s + 1 - f);
    return biop(st[diff][f], st[diff][s + 1 - (1 << diff)]);
  }
};


#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<PI, int> PIPI;

struct pmax {
  PI operator()(PI x, PI y) const {
    return max(x, y);
  }
};


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI a(n);
  vector<PI> pool(n + 1);
  REP(i, 0, n) {
    cin >> a[i];
    pool[i] = PI(a[i], i);
  }
  pool[n] = PI(0, n);
  sort(pool.rbegin(), pool.rend());
  vector<PI> diff(n);
  REP(i, 0, n) {
    diff[i] = PI(pool[i].first - pool[i + 1].first, -(i + 1));
  }
  SparseTable<PI, pmax> spt(pmax(), diff);

  PIPI ma(PI(-1, -1), -1);
  PIPI real(PI(-1, -1), -1);
  REP(a2, 1, n - 1) {
    int l1 = a2;
    int d1c2 = pool[a2 - 1].first - pool[a2].first;
    if (ma.first.first > d1c2) { continue; }
    REP(a3, a2 + 1, n) {
      int d2c3 = pool[a3 - 1].first - pool[a3].first;
      PI two(d1c2, d2c3);
      if (ma.first > two) { continue; }
      int l2 = a3 - a2;
      if (l1 > 2 * l2 || l2 > 2 * l1) { continue; }
      int l3mi = max((l1 + 1) / 2, (l2 + 1) / 2);
      int l3ma = min(2 * l1, 2 * l2);
      int a4mi = a3 + l3mi;
      int a4ma = min(a3 + l3ma, n);
      if (a4mi > a4ma) { continue; }
      assert (a4mi > a3);
      assert (a4ma <= n);
      PI res = spt.query(a4mi - 1, a4ma - 1);
      if (ma < PIPI(two, res.first)) {
	ma = PIPI(two, res.first);
	int idx = -res.second;
	real = PIPI(PI(a2, a3), idx);
      }
    }
  }
  VI ans(n);
  int a2 = real.first.first;
  int a3 = real.first.second;
  int a4 = real.second;
  REP(i, 0, a2) {
    ans[pool[i].second] = 1;
  }
  REP(i, a2, a3) {
    ans[pool[i].second] = 2;
  }
  REP(i, a3, a4) {
    ans[pool[i].second] = 3;
  }
  REP(i, a4, n) {
    ans[pool[i].second] = -1;
  }
  
  REP(i, 0, n) {
    cout << ans[i] << (i == n - 1 ? "\n" : " ");
  }
}
