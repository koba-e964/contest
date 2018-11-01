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

/**
 * Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid I. Note that constructing this tree requires the identity
 * element of I and the operation of I.
 * Header requirement: vector, algorithm
 * Verified by ABC017-D (http://abc017.contest.atcoder.jp/submissions/660402)
 *             AGC009-C (http://agc009.contest.atcoder.jp/submissions/1461012)
 */
template<class I, class BiOp>
class SegTree {
  int n;
  std::vector<I> dat;
  std::vector<I> dat2; // max
  BiOp op;
  I e;
public:
  SegTree(int n_, BiOp op, I e) : op(op), e(e) {
    n = 1;
    while (n < n_) { n *= 2; } // n is a power of 2
    dat.resize(2 * n);
    dat2.resize(2 * n);
    for (int i = 0; i < 2 * n - 1; i++) {
      dat[i] = e;
      dat2[i] = e;
    }
  }
  /* ary[k] <- v */
  void update(int k, I v) {
    k += n - 1;
    dat[k] = v;
    dat2[k] = v;
    while (k > 0) {
      k = (k - 1) / 2;
      dat[k] = op(dat[2 * k + 1], dat[2 * k + 2]);
      dat2[k] = std::max(dat2[2 * k + 1], dat2[2 * k + 2]);
    }
  }
  /* http://proc-cpuinfo.fixstars.com/2017/07/optimize-segment-tree/ */
  I querySub(int a, int b) const {
    I left = e;
    I right = e;
    a += n - 1;
    b += n - 1;
    while (a < b) {
      if ((a & 1) == 0) {
	left = op(left, dat[a]);
      }
      if ((b & 1) == 0) {
	right = op(dat[b - 1], right);
      }
      a = a / 2;
      b = (b - 1) / 2;
    }
    return op(left, right);
  }
  /* [a, b] (note: inclusive) */
  I query(int a, int b) const {
    return querySub(a, b + 1);
  }
  void queryRemainder(int a, int b, int k, int l, int r, int x) {
    if (dat2[k] < x) return;
    if (r <= a || b <= l) return;
    if (r - l <= 1) {
      dat[k] %= x;
      dat2[k] = dat[k];
      return;
    }
    int mid = (l + r) / 2;
    queryRemainder(a, b, 2 * k + 1, l, mid, x);
    queryRemainder(a, b, 2 * k + 2, mid, r, x);
    dat[k] = op(dat[2 * k + 1], dat[2 * k + 2]);
    dat2[k] = std::max(dat2[2 * k + 1], dat2[2 * k + 2]);
  }
  // [a, b]
  void queryRemainder(int a, int b, int x) {
    queryRemainder(a, b + 1, 0, 0, n, x);
  }
};


using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

struct pmax {
  int operator()(int a, int b) const {
    return max(a, b);
  }
};

// The author read the editorial.
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  VI a(n);
  REP(i, 0, n) cin >> a[i];
  SegTree<ll, plus<ll> > st(n, plus<ll>(), 0);
  REP(i, 0, n) {
    st.update(i, a[i]);
  }
  REP(i, 0, m) {
    int ty;
    cin >> ty;
    if (ty == 1) {
      int l, r;
      cin >> l >> r;
      l--, r--;
      cout << st.query(l, r) << "\n";
    } else if (ty == 3) {
      int k, x;
      cin >> k >> x;
      k--;
      st.update(k, x);
    } else {
      int l, r, x;
      cin >> l >> r >> x;
      l--, r--;
      st.queryRemainder(l, r, x);
    }
  }
}
