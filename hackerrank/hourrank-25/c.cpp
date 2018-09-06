#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>
#include <map>
#include <cassert>

using namespace std;

const int DEBUG = 0;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
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

struct pmax {
  int operator()(int x, int y) const {
    return max(x, y);
  }
};
struct pmin {
  ll operator()(ll x, ll y) const {
    return min(x, y);
  }
};

int gcd(int x, int y) {
  x = abs(x);
  y = abs(y);
  while (y != 0) {
    int r = x % y;
    x = y; y = r;
  }
  return x;
}

const ll inf = 1e17;

int main(void) {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI a(n);
  REP(i, 0, n) cin >> a[i];
  VL acc(n + 1);
  REP(i, 0, n) acc[i + 1] = acc[i] + a[i];
  SparseTable<int, pmax> spt(pmax(), a);
  SparseTable<ll, pmin> spt_acc(pmin(), acc);
  map<int, ll> win;
  map<PI, ll> opt;
  ll ma = 0;
  REP(i, 0, n) {
    int aa = abs(a[i]);
    int pass = -1, fail = i;
    while (fail - pass > 1) {
      int mid = (pass + fail) / 2;
      if (spt.query(mid, i - 1) > a[i]) {
	pass = mid;
      } else {
	fail = mid;
      }
    }
    map<int, ll> nwin;
    nwin[aa] = i - n;
    for (auto e: win) {
      int g = gcd(e.first, a[i]);
      nwin[g] = min(nwin[g], e.second);
    }
    win = nwin;
    for (auto e: win) {
      ll g = e.first;
      int start = e.second + n;
      if (DEBUG)
        cerr << "i = " << i << " g = " << g << " start = " << start << endl;
      ll newval = acc[i] - spt_acc.query(max(start, fail), i);
      if (start <= pass) {
	newval = max(newval, opt[PI(start, pass)] + acc[i + 1] - acc[pass + 1]);
      }
      opt[PI(start, i)] = newval;
      if (DEBUG) {
	cerr << "newval = " << newval << endl;
      }
      if (newval >= 0) {
        ma = max(ma, g * newval);
      }
    }
  }
  cout << ma << endl;
}
