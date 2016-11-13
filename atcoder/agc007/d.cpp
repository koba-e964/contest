#include <algorithm>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<ll> VL;

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
const ll inf = 1e16;

struct min_fun {
  ll operator()(ll x, ll y) const {
    return min(x, y);
  }
};

int main(void){
  int n;
  ll e, t;
  cin >> n >> e >> t;
  VL x(n);
  SegTree<ll, min_fun> raw(n + 1, min_fun(), inf); // dp[i]
  SegTree<ll, min_fun> boiled(n + 1, min_fun(), inf); // dp[i] - 2 * x[i]
  REP(i, 0, n) {
    cin >> x[i];
  }
  raw.update(0, 0);
  boiled.update(0, -2 * x[0]);
  REP(i, 0, n) {
    // Compute the range of j s.t. t <= 2 * (x[i] - x[j]) (0 <= j <= i)
    int j_boundary =
      upper_bound(x.begin(), x.begin() + i + 1, x[i] - (t+1)/2) - x.begin();
    ll ret = inf;
    ret = min(ret, boiled.query(0, j_boundary - 1) + 2 * x[i]);
    ret = min(ret, raw.query(j_boundary, i) + t);
    raw.update(i + 1, ret);
    if (i < n - 1) {
      boiled.update(i + 1, ret - 2 * x[i + 1]);
    }
  }
  cout << raw.query(n, n) + e << endl; 
}
