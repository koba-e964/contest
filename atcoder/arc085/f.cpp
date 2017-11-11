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

/**
 * Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid I. Note that constructing this tree requires the identity
 * element of I and the operation of I.
 * Header requirement: vector, algorithm
 * Verified by ABC017-D (http://abc017.contest.atcoder.jp/submissions/660402)
 *             AGC009-C (http://agc009.contest.atcoder.jp/submissions/1461012)
 */
template<class I, class BiOp = I (*) (I, I)>
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
  void update_array(int k, int len, const I *vals) {
    for (int i = 0; i < len; ++i) {
      update(k + i, vals[i]);
    }
  }
  /*
    Updates all elements. O(n)
   */
  void update_all(const I *vals, int len) {
    for (int k = 0; k < std::min(n, len); ++k) {
      dat[k + n - 1] = vals[k];
    }
    for (int k = std::min(n, len); k < n; ++k) {
      dat[k + n - 1] = e;
    }
    for (int b = n / 2; b >= 1; b /= 2) {
      for (int k = 0; k < b; ++k) {
	dat[k + b - 1] = op(dat[k * 2 + b * 2 - 1], dat[k * 2 + b * 2]);
      }
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
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

struct pmin {
  ll operator()(ll x, ll y) const {
    return min(x, y);
  }
};

const ll inf = 1e16;

const int DEBUG = 0;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI b(n);
  VL bc(n + 1, 0);
  REP(i, 0, n) {
    cin >> b[i];
    bc[i + 1] = bc[i] + b[i];
  }
  int q;
  cin >> q;
  vector<PI> pool;
  REP(i, 0, q) {
    int l, r;
    cin >> l >> r;
    l--;
    // [l, r)
    pool.push_back(PI(l, r));
  }
  sort(pool.begin(), pool.end());
  SegTree<ll, pmin> st(n + 1, pmin(), inf);
  SegTree<ll, pmin> stp(n + 1, pmin(), inf);
  VL dp(n + 1, inf);
  dp[0] = 0;
  st.update(0, 0);
  stp.update(0, 0);
  REP(i, 0, q) {
    int l = pool[i].first;
    int r = pool[i].second;
    ll val = stp.query(l, r - 1) + r - bc[r];
    if (DEBUG) {
      cerr << "l,r=" << l << " " << r << endl;
      cerr << "val = " << val << endl;
    }
    if (l >= 1) {
      val = min(val, st.query(0, l - 1) + bc[l] + r - l - bc[r] + bc[l]);
    }
    val = min(dp[r], val);
    dp[r] = val;
    st.update(r, val - bc[r]);
    stp.update(r, val + bc[r] - r);
  }
  if (DEBUG) {
    cerr << "dp:";
    REP(i, 0, n + 1) {
      cerr << " " << dp[i];
    }
    cerr << endl;
  }
  ll mi = inf;
  REP(i, 0, n + 1) {
    mi = min(mi, dp[i] + bc[n] - bc[i]);
  }
  cout << mi << "\n";
}
