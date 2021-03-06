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
 
struct pmax {
  ll operator()(ll x, ll y) const {
    return max(x, y);
  }
};
 
 
const ll inf = 5e15; // 5000兆
 
int main(void){
  int n;
  ll p;
  cin >> n >> p;
  VL x(n), acc(n + 1, 0);
  // acc[0] = 0, acc[i] = x[0] + ... x[i - 1]
  REP(i, 0, n) {
    cin >> x[i];
    acc[i + 1] = acc[i] + x[i];
  }
  // for each i, compute the maximum k s.t. acc[i + k] - acc[i] = x[i] + ... x[i + k - 1] >= p
  // Check if max(acc[i + l] - acc[i], ..., acc[n] - acc[i]) >= p.
  //  <=>  max(acc[i + l], ..., acc[n]) >= acc[i] + p.
  // The maximum l that satisfies it is what we want.
  // This can be calculated by binary search on a segtree.
  SegTree<ll, pmax> st(n + 1, pmax(), -inf);
  REP(i, 0, n + 1) {
    st.update(i, acc[i]);
  }
  int ma = 0;
  REP(i, 0, n) {
    int lo = i;
    int hi = n + 1;
    while (hi - lo > 1) {
      int mid = (hi + lo) / 2;
      if (st.query(mid, n) >= acc[i] + p) {
        lo = mid;
      } else {
        hi = mid;
      }
    }
    ma = max(ma, lo - i);
  }
  cout << ma << endl;
}
