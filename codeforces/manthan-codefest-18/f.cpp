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

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;
/*
 * Dependencies: typedef long long ll
 * Headers: iostream
 * Verified by: ARC099-F
 *              https://beta.atcoder.jp/contests/arc099/submissions/2743855
 */

template<ll mod = (ll)1e9 + 7>
struct ModInt {
  ll x;
  ModInt(void): x(0) {}
  ModInt(ll x): x(x % mod){}
  ModInt(const ModInt &x): x(x.x) {}
  ModInt operator+(ModInt o) const {
    ll y = x + o.x;
    if (y >= mod) y -= mod;
    return ModInt(y);
  }
  ModInt operator-(ModInt o) const {
    ll y = x - o.x + mod;
    if (y >= mod) y -= mod;
    return ModInt(y);
  }
  ModInt operator*(ModInt o) const {
    return ModInt((x * o.x) % mod);
  }
  void operator+=(ModInt o) { *this = *this + o; }
  void operator-=(ModInt o) { *this = *this - o; }
  void operator*=(ModInt o) { *this = *this * o; }
  ModInt operator-(void) const { return ModInt() - *this; }
  ll to_ll() const {
    return x;
  }
  bool operator<(ModInt o) const {
    return x < o.x;
  }
  ModInt pow(ll e) {
    assert (e >= 0);
    ModInt sum = 1;
    ModInt cur = *this;
    while (e > 0) {
      if (e % 2) {
        sum = sum * cur;
      }
      cur = cur * cur;
      e /= 2;
    }
    return sum;
  }
  ModInt inv(void) {
    return pow(mod - 2);
  }
};

template<ll mod>
ostream &operator<<(ostream &os, ModInt<mod> mi) {
  return os << mi.x;
}

/**
 * Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid I. Note that constructing this tree requires the identity
 * element of I and the operation of I.
 * Header requirement: vector, algorithm
 * Verified by ABC017-D (http://abc017.contest.atcoder.jp/submissions/660402)
 *             AGC009-C (http://agc009.contest.atcoder.jp/submissions/1461012)
 */
template<class I, class BiOp>
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


typedef pair<ll, PI> PLPI;
struct pmax {
  PI operator()(PI x, PI y) const {
    return max(x, y);
  }
};

// [-l..r]
ModInt<> calc(int l, int r, int k) {
  if (l < r) swap(l, r);
  ModInt<> tot;
  // r >= i(k - 1)
  {
    ll lim = r / (k - 1);
    tot += ModInt<>(lim * (lim + 1) / 2) * (k - 1);
    tot += lim;
  }
  // r < i(k - 1) <= l
  {
    ll cnt = l / (k - 1) - r / (k - 1);
    tot += cnt * (r + 1);
  }
  // l < i(k - 1)
  {
    // (lo, lim]
    ll lim = (l + r + 1) / (k - 1);
    ll lo = l / (k - 1);
    tot += (lim - lo) * (l + r + 1);
    tot -= ModInt<>((lim * (lim + 1) - lo * (lo + 1)) / 2) * (k - 1);
  }
  return tot;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k;
  cin >> n >> k;
  VI a(n);
  REP(i, 0, n) cin >> a[i];
  SegTree<PI, pmax> st(n, pmax(), PI(-1, -1));
  REP(i, 0, n) st.update(i, PI(a[i], i));
  int ops = (n - 1) / (k - 1);
  priority_queue<PLPI> que;
  que.push(PLPI(st.query(0, n - 1).first, PI(0, n - 1)));
  ModInt<> tot;
  while (not que.empty()) {
    PLPI t = que.top(); que.pop();
    ll v = t.first;
    PI seg = t.second;
    if (DEBUG) {
      cerr << "[" << seg.first << ", " << seg.second << "]" << "(max = " << v <<
	")" << endl;
    }
    int idx = st.query(seg.first, seg.second).second;
    tot += calc(idx - seg.first, seg.second - idx, k) * ModInt<>(v);
    if (seg.first < idx) {
      que.push(PLPI(st.query(seg.first, idx - 1).first, PI(seg.first, idx - 1)));
    }
    if (idx < seg.second) {
      que.push(PLPI(st.query(idx + 1, seg.second).first, PI(idx + 1, seg.second)));
    }
  }
  cout << tot << endl;
}
