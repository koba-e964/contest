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


#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;

// Omosiroi monoid kozo
struct omo {
  ll a[3];
  omo() {}
  omo(ll x, ll y, ll z) {
    a[0] = x;
    a[1] = y;
    a[2] = z;
  }
};
struct omomon {
  omo operator()(omo x, omo y) const {
    ll com = min(x.a[1], y.a[2]);
    omo ret;
    ret.a[0] = x.a[0] + y.a[0] + com;
    ret.a[1] = x.a[1] + y.a[1] - com;
    ret.a[2] = x.a[2] + y.a[2] - com;
    return ret;
  }
};

// Reference: https://twitter.com/btk15049/status/838389594072592385
int main(void){
  int q;
  ll k;
  cin >> q >> k;
  vector<PL> qs;
  REP(i, 0, q) {
    int ty;
    cin >> ty;
    if (ty == 1) {
      ll d, a;
      cin >> d >> a;
      qs.push_back(PL(d, a));
    } else {
      ll d;
      cin >> d;
      qs.push_back(PL(d, -1LL));
    }
  }
  // coord comp
  set<ll> inv;
  inv.insert(0);
  REP(i, 0, q) {
    inv.insert(qs[i].first);
  }
  vector<ll> inv_v(inv.begin(), inv.end());
  sort(inv_v.begin(), inv_v.end());
  int m = inv_v.size();
  map<ll, int> corr;
  REP(i, 0, m) {
    corr[inv_v[i]] = i;
  }
  assert (corr[0] == 0);
  REP(i, 0, q) {
    qs[i].first = corr[qs[i].first];
  }
  SegTree<omo, omomon> st(m, omomon(), omo(0, 0, 0));
  REP(i, 1, m) {
    ll wert = (inv_v[i] - inv_v[i - 1]) * k;
    st.update(i, omo(0, wert, 0));
  }
  vector<bool> init(m, false);
  REP(i, 0, q) {
    // process queries
    if (qs[i].second == -1) {
      omo qn = st.query(0, qs[i].first);
      cout << qn.a[0] << endl;
    } else {
      int time = qs[i].first;
      ll a = qs[i].second;
      omo former = st.query(time, time);
      ll b = min(a, former.a[1]);
      former.a[0] += b;
      former.a[1] -= b;
      a -= b;
      former.a[2] += a;
      st.update(time, former);
    }
  }
}
