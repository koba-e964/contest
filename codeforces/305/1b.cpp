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

struct pmin {
  ll operator()(ll x, ll y) const {
    return min(x, y);
  }
};


const ll inf = 5e15; // 5000-chou

int main(void){
  cin.tie(0);
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  SparseTable<ll, pmin> spt(pmin(), a);
  VL tbl(n + 1, -inf);
  REP(i, 0, n) {
    // max t s.t. spt.query(i, t) == a[i]
    int hi = n;
    int lo = i;
    while (hi - lo > 1) {
      int mid = (hi + lo) / 2;
      if (spt.query(i, mid) == a[i]) {
	lo = mid;
      } else {
	hi = mid;
      }
    }
    int t = lo;
    // min u s.t. spt.query(u, i) == a[i]
    hi = i;
    lo = -1;
    while (hi - lo > 1) {
      int mid = (hi + lo) / 2;
      if (spt.query(mid, i) == a[i]) {
	hi = mid;
      } else {
	lo = mid;
      }
    }
    int u = hi;
    tbl[t - u + 1] = max(tbl[t - u + 1], a[i]);
  }
  for (int i = n - 1; i >= 0; --i) {
    tbl[i] = max(tbl[i], tbl[i + 1]);
  }
  REP(i, 1, n + 1) {
    cout << tbl[i] << (i == n ? "\n" : " ");
  }
}
