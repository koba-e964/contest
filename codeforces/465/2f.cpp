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


struct pmin {
  ll operator()(ll x, ll y) const {
    return min(x, y);
  }
};

const ll inf = 1e16;

const int N = 100100;
ll a[N], b[N];
ll btot = 0;
SegTree<ll, pmin> *st;

void upd_seg(int idx) {
  ll u = b[idx];
  ll v = b[idx + 1];
  ll val = 0;
  if (u >= 0 && v <= 0) {
    val = 0;
  } else if (u >= 0 && v >= 0) {
    val = v;
  } else if (u <= 0 && v <= 0) {
    val = -u;
  } else {
    val = -u + v;
  }
  st->update(idx, val);
}

void add(int idx, ll val) {
  btot -= abs(b[idx]);
  b[idx] += val;
  btot += abs(b[idx]);
  upd_seg(idx);
  upd_seg(idx - 1);
}



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, q;
  cin >> n;
  REP(i, 0, n) cin >> a[i];
  REP(i, 0, n) {
    b[i] = a[i] - (i == 0 ? 0 : a[i - 1]);
  }
  cin >> q;
  REP(i, 1, n) btot += abs(b[i]);
  st = new SegTree<ll, pmin>(n, pmin(), inf);
  REP(i, 0, n - 1) {
    upd_seg(i);
  }
  REP(i, 0, q) {
    int t, l, r;
    ll x;
    cin >> t >> l >> r >> x;
    l--, r--;
    if (t == 2) {
      add(r + 1, -x);
      add(l, x);
      continue;
    }
    ll ma = -inf;
    if (r - l <= 1) {
      REP(i, l, r + 1) {
	ll diff = abs(b[i] + x) + abs(b[i + 1] - x) - (abs(b[i]) + abs(b[i + 1]));
	ma = max(ma, diff);
      }
    } else {
      ll segm = st->query(l, r);
      ma = max(0LL, 2 * (x - segm));
      int tgs[2] = {l, r};
      REP(dir, 0, 2) {
	int i = tgs[dir];
	ll diff = abs(b[i] + x) + abs(b[i + 1] - x) - (abs(b[i]) + abs(b[i + 1]));
	ma = max(ma, diff);
      }
    }
    cout << btot + ma << "\n";
  }
}
