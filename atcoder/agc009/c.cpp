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

struct modp {
  ll operator()(ll a, ll b) const {
    return (a + b) % mod;
  }
};

const int DEBUG = 0;

int main(void){
  int n;
  ll a, b;
  cin >> n >> a >> b;
  VL s(n);
  REP(i, 0, n) {
    cin >> s[i];
  }
  SegTree<ll, modp> st1(n + 1, modp(), 0), st2(n + 1, modp(), 0);
  st1.update(0, 1);
  st2.update(0, 1);
  int u = 0, v = 0; // range of 0.
  REP(i, 1, n) {
    // binsect
    int u2 = upper_bound(s.begin(), s.begin() + i, s[i] - a) - s.begin();
    // += st2[0 .. u2].sum()
    ll st1add = st2.query(0, u2);
    // binsect
    int v2 = upper_bound(s.begin(), s.begin() + i, s[i] - b) - s.begin();
    ll st2add = st1.query(0, v2);
    // += st2[0 .. v2].sum()
    st1.update(i, st1add + st1.query(i, i));
    st2.update(i, st2add + st2.query(i, i));

    // zero-fill
    if (s[i] - s[i - 1] < a) { // st1
      while (u < i) {
	st1.update(u, 0);
	u++;
      }
    }
    if (s[i] - s[i - 1] < b) { // st2
      while (v < i) {
	st2.update(v, 0);
	v++;
      }
    }
    if (DEBUG) {
      cerr << "st1:";
      REP(j, 0, i + 2) {
	cerr << " " << st1.query(j, j);
      }
      cerr << endl;
      cerr << "st2:";
      REP(j, 0, i + 2) {
	cerr << " " << st2.query(j, j);
      }
      cerr << endl;
    }
  }
  ll sum = st1.query(0, n);
  sum = sum + st2.query(0, n);
  sum %= mod;
  cout << sum << endl;
}
