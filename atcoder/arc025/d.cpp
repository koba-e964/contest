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


using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int W = 100100;
ll pow2[W];
ll fib[W];

int main(void){
  int h, w, n;
  cin >> h >> w >> n;
  vector<PI> st;
  REP(i, 0, n) {
    int s, t;
    cin >> s >> t;
    s--, t--;
    st.push_back(PI(s, t));
  }
  pow2[0] = 1;
  REP(i, 1, W) {
    pow2[i] = pow2[i - 1] * 2 % mod;
  }
  fib[0] = fib[1] = 1;
  REP(i, 2, W) {
    fib[i] = (fib[i - 1] + fib[i - 2]) % mod;
  }
  assert (w <= 5000 && n <= 3000);
  if (h == 1) {
    VI board(w + 1, 0);
    board[w] = 1;
    REP(i, 0, n) {
      board[st[i].second] ^= 1;
      ll sum = 1;
      int cur = 0;
      REP(j, 0, w + 1) {
	if (board[j] == 1) {
	  sum = sum * fib[cur] % mod;
	  cur = 0;
	} else {
	  cur++;
	}
      }
      cout << sum << endl;
    }
    return 0;
  }
  // h == 2
  VI board(w, 0);
  REP(i, 0, n) {
    board[st[i].second] ^= 1 << st[i].first;
    vector<VL> dp(w + 1, VL(4, 0));
    dp[0][3] = 1;
    REP(j, 1, w + 1) {
      int cb = board[j - 1];
      REP(b, 0, 4) {
	dp[j][cb] += dp[j - 1][b];
	dp[j][cb] %= mod;
      }
      if (cb == 0) {
	dp[j][3] = dp[j][0];
	dp[j][3] += dp[j - 1][0];
	dp[j][3] %= mod;
      }
      REP(b, 1, 3) {
	if ((cb & b) == 0) { // fill 0 horizontally
	  dp[j][cb | b] += dp[j - 1][0];
	  dp[j][cb | b] += dp[j - 1][3 - b];
	  dp[j][cb | b] %= mod;
	}
      }
    }
    ll tot = 0;
    REP(b, 0, 4) {
      tot += dp[w][b];
    }
    cout << tot % mod << endl;
  }
}
