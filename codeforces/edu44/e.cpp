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
const ll mod = 1e9 + 7;

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


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k;
  ll d;
  cin >> n >> k >> d;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  sort(a.begin(), a.end());
  SegTree<int, plus<int> > st(n + 1, plus<int>(), 0);
  VI dp(n + 1);
  dp[0] = 1;
  st.update(0, 1);
  REP(i, k, n + 1) {
    int idx = lower_bound(a.begin(), a.begin() + i - k + 1, a[i - 1] - d) - a.begin();
    if (idx >= i - k + 1) continue;
    int res = st.query(idx, i - k);
    if (res) {
      st.update(i, 1);
      dp[i] = 1;
    }
  }
  cout << (dp[n] ? "YES" : "NO") << endl;
}
