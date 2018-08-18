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

struct pmin {
  int operator()(int a, int b) const {
    return min(a, b);
  }
};

const int inf = 1e8;
void fail(void) {
  cout << "NO\n";
  exit(0);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, q;
  cin >> n >> q;
  VI a(n);
  REP(i, 0, n) cin >> a[i];
  VI ma(q + 1, -inf), mi(q + 1, inf);
  REP(i, 0, n) {
    int v = a[i];
    ma[v] = max(ma[v], i);
    mi[v] = min(mi[v], i);
  }
  if (ma[q] < mi[q] && ma[0] < mi[0]) fail();
  if (ma[q] < mi[q]) {
    a[mi[0]] = q;
  }
  REP(i, 1, n) {
    if (a[i] == 0) {
      a[i] = a[i - 1];
    }
  }
  for (int i = n - 2; i >= 0; --i) {
    if (a[i] == 0) a[i] = a[i + 1];
  }
  REP(i, 0, n) assert (a[i] != 0);
  SegTree<int, pmin> st(n, pmin(), inf);
  REP(i, 0, n) st.update(i, a[i]);
  REP(i, 0, q + 1) {
    mi[i] = inf;
    ma[i] = -inf;
  }
  REP(i, 0, n) {
    int v = a[i];
    ma[v] = max(ma[v], i);
    mi[v] = min(mi[v], i);
  }
  for (int i = q; i >= 1; --i) {
    if (ma[i] < mi[i]) continue;
    if (st.query(mi[i], ma[i]) < i) fail();
  }
  cout << "YES\n";
  REP(i, 0, n) cout << a[i] << (i == n - 1 ? "\n" : " ");
}
