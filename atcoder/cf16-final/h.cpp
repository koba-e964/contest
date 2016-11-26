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
const ll mod = 1e9 + 7;

/**
 * Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid M. Note that constructing this tree requires the identity
 * element of M and the operation of M.
 * Header requirement: vector, algorithm
 * Verified by AtCoder ABC017-D (http://abc017.contest.atcoder.jp/submissions/660402)
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
  /* l,r are for simplicity */
  I querySub(int a, int b, int k, int l, int r) const {
    // [a,b) and  [l,r) intersects?
    if (r <= a || b <= l) return e;
    if (a <= l && r <= b) return dat[k];
    I vl = querySub(a, b, 2 * k + 1, l, (l + r) / 2);
    I vr = querySub(a, b, 2 * k + 2, (l + r) / 2, r);
    return op(vl, vr);
  }
  /* [a, b] (note: inclusive) */
  I query(int a, int b) const {
    return querySub(a, b + 1, 0, 0, n);
  }
};


const int N = 200100;

struct max_fun {
  ll operator()(ll x, ll y) const {
    return max(x, y);
  }
};

ll solve(const VL &a) {
  int n = a.size();
  VL acc(n + 1), diff(n);
  acc[0] = 0;
  diff[0] = a[0];
  REP(i, 1, n + 1) {
    acc[i] = acc[i - 1] + a[i - 1];
    if (i < n) {
      diff[i] = a[i] - acc[i];
    }
  }
  // r = s + 1
  // find max -dp[i - 1] + (a[i] - acc[i] + acc[r+1]) (i >= r + 1)
  SegTree<ll, max_fun> st(n, max_fun(), -1e16);
  VL dp(n - 1);
  dp[n - 2] = 0;
  st.update(n - 1, a[n - 1] - acc[n - 1]);
  for (int s = n - 3; s >= 0; --s) {
    ll res = st.query(s + 2, n - 1);
    dp[s] = res + acc[s + 2];
    st.update(s + 1, a[s + 1] - acc[s + 1] - dp[s]);
  }
  if (0) {
    REP(i, 0, n - 1) {
      cerr << "dp[" << i << "]=" << dp[i] << endl;
    }
    REP(i, 0, n) {
      cerr << "a[" << i << "]=" << a[i] << ", acc[" << i << "]=" << acc[i] << endl;
    }
    REP(i, 0, n) {
      cerr << "st[" << i << "]=" << st.query(i, i) << endl;
    }
  }
  return dp[0];
}

int main(void){
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n - 1) {
    cin >> a[i];
  }
  int m;
  cin >> m;
  VL x(m);
  REP(i, 0, m) {
    cin >> x[i];
  }
  assert (m <= 10);
  REP(j, 0, m) {
    a[n - 1] = x[j];
    ll res = solve(a);
    cout << res + a[0] - a[1] << endl;
  }
}
