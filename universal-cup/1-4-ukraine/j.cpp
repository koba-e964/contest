#include <algorithm>
#include <iomanip>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

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
};

struct BXor {
    bool operator()(bool a, bool b) const {
        return a ^ b;
    }
};

// Solved with hints
int main(void){
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, q;
  cin >> n >> q;
  vector<int> p(n);
  REP(i, 0, n) {
    cin >> p[i];
    p[i]--;
  }
  SegTree<bool, BXor> st(n, BXor(), false);
  vector<bool> l1(n); // leave-one
  bool inv = false;
  REP(i, 0, n) {
    bool x = st.querySub(p[i] + 1, n);
    inv ^= x;
    l1[i] = l1[i] ^ x;
    st.update(p[i], true);
  }
  st = SegTree<bool, BXor>(n, BXor(), false);
  for (int i = n - 1; i >= 0; i--) {
    bool x = st.querySub(0, p[i]);
    l1[i] = l1[i] ^ x;
    st.update(p[i], true);
  }
  int tc = 0;
  int did = 0;
  REP(i, 0, n) {
    if (l1[i]) tc++;
    if (p[i] != i) did++;
  }
  while (q--) {
    int u, v;
    cin >> u >> v;
    u--, v--;
    inv = !inv;
    if (p[u] != u) did--;
    if (p[v] != v) did--;
    swap(p[u], p[v]);
    l1.swap(l1[u], l1[v]);
    if (p[u] != u) did++;
    if (p[v] != v) did++;
    if ((u + v) % 2) {
        l1[u] = !l1[u];
        tc += 2 * l1[u] - 1;
        l1[v] = !l1[v];
        tc += 2 * l1[v] - 1;
    }
    if (did == 0) {
        cout << "-1\n";
    } else if (inv) {
        cout << n << "\n";
    } else if (tc) {
        cout << n - 1 << "\n";
    } else {
        cout << n - 2 << "\n";
    }
  }
}
