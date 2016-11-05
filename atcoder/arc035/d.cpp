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


const int  N = 200100;
int p[N], q[N];

const int T = 2001000;
double log_fact[T];

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> p[i] >> q[i];
  }
  int nq;
  cin >> nq;
  log_fact[0] = 0.0;
  REP(i, 1, T) {
    log_fact[i] = log_fact[i - 1] + log(i);
  }
  SegTree<double, plus<double> > st(n - 1, plus<double>(), 0);
  REP(i, 0, n - 1) {
    int x = p[i + 1] - p[i];
    int y = q[i + 1] - q[i];
    st.update(i, log_fact[x + y] - log_fact[x] - log_fact[y]);
  }
  REP(loop_cnt, 0, nq) {
    int t;
    cin >> t;
    if (t == 1) {
      int k, a, b;
      cin >> k >> a >> b;
      k--;
      p[k] = a;
      q[k] = b;
      if (k >= 1) {
	int x = p[k] - p[k - 1];
	int y = q[k] - q[k - 1];
	st.update(k - 1, log_fact[x + y] - log_fact[x] - log_fact[y]);
      }
      if (k < n - 1) {
	int x = p[k + 1] - p[k];
	int y = q[k + 1] - q[k];
	st.update(k, log_fact[x + y] - log_fact[x] - log_fact[y]);
      }
    } else {
      int l1, r1, l2, r2;
      cin >> l1 >> r1 >> l2 >> r2;
      double first = st.query(l1 - 1, r1 - 2);
      double second = st.query(l2 - 1, r2 - 2);
      cout << (first > second ? "FIRST" : "SECOND") << endl;
    }
  }
}
