#include <cassert>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;


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

struct pmax {
  int operator()(int x, int y) const {
    return max(x, y);
  }
};

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  SparseTable<int, bit_or<int> > spt_or(bit_or<int>(), a);
  SparseTable<int, pmax> spt_max(pmax(), a);
  ll tot = 0;
  REP(i, 0, n) {
    int start, end;
    int lo = -1;
    int hi = i;
    while (hi - lo > 1) {
      int mid = (hi + lo) / 2;
      if (spt_max.query(mid, i - 1) < a[i]) {
	hi = mid;
      } else {
        lo = mid;
      }
    }
    start = hi;
    lo = i;
    hi = n;
    while (hi - lo > 1) {
      int mid = (hi + lo) / 2;
      if (spt_max.query(i, mid) == a[i]) {
	lo = mid;
      } else {
	hi = mid;
      }
    }
    end = lo;
    int left, right;
    lo = start - 1;
    hi = i;
    while (hi - lo > 1) {
      int mid = (hi + lo) / 2;
      if (spt_or.query(mid, i) > a[i]) {
	lo = mid;
      } else {
	hi = mid;
      }
    }
    left = hi;
    lo = i;
    hi = end + 1;
    while (hi - lo > 1) {
      int mid = (hi + lo) / 2;
      if (spt_or.query(i, mid) > a[i]) {
	hi = mid;
      } else {
	lo = mid;
      }
    }
    right = lo;
    if (0) {
      cerr << "a[" << i << "]=" << a[i] << endl;
      cerr << "range: [" << start << ", " << end << "]" << endl;
      cerr << "left = " << left << " right = " << right << endl;
    }
    tot += ll(end - i + 1) * ll(i - start + 1);
    tot -= ll(right - i + 1) * ll(i - left + 1);
  }
  cout << tot << "\n";
}
