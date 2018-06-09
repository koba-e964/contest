#include <iostream>
#include <vector>
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
  BiOp op;
  I e;
public:
  SegTree(int n_, BiOp op, I e) : op(op), e(e) {
    n = 1;
    while (n < n_) { n *= 2; } // n is a power of 2
    dat.resize(2 * n);
    for (int i = 0; i < 2 * n - 1; i++) {
      dat[i] = e;
    }
  }
  /* ary[k] <- v */
  void update(int k, I v) {
    k += n - 1;
    dat[k] = v;
    while (k > 0) {
      k = (k - 1) / 2;
      dat[k] = op(dat[2 * k + 1], dat[2 * k + 2]);
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
};

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, q;
  cin >> n >> q;
  int lft = 0;
  int rgt = n;
  bool rev = false;
  SegTree<int, plus<int> > st(n, plus<int>(), 0);
  REP(i, 0, n) st.update(i, 1);
  REP(_, 0, q) {
    int len = rgt - lft;
    int ty;
    cin >> ty;
    if (ty == 1) {
      int p;
      cin >> p;
      if (2 * p > len) {
	rev = not rev;
	p = len - p;
      }
      if (rev) {
	REP(i, 0, p) {
	  int val = st.query(rgt - 1 - i, rgt - 1 - i);
	  st.update(rgt - 2 * p + i, st.query(rgt - 2 * p + i, rgt - 2 * p + i) + val);
	  st.update(rgt - 1 - i, 0);
	}
	rgt = rgt - p;
      } else {
	REP(i, 0, p) {
	  int val = st.query(lft + i, lft + i);
	  st.update(lft + 2 * p - 1 - i, st.query(lft + 2 * p - 1 - i, lft + 2 * p - 1 - i) + val);
	  st.update(lft + i, 0);
	}
	lft = lft + p;
      }
    } else {
      int l, r;
      cin >> l >> r;
      int ans = st.query(lft + (rev ? len - r : l), lft + (rev ? len - l : r) - 1);
      cout << ans << "\n";
    }
  }
}
