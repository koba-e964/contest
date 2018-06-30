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

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string s;
  cin >> s;
  int n = s.length();
  VI p(n + 1);
  REP(i, 0, n) {
    p[i + 1] = p[i] + (s[i] == '(' ? 1 : -1);
  }
  SegTree<int, pmin> st(n + 1, pmin(), 1e8);
  REP(i, 0, n + 1) {
    st.update(i, p[i]);
  }
  vector<VI> tap(2 * n + 1);
  REP(i, 0, n + 1) {
    int idx = p[i] + n;
    tap[idx].push_back(i);
  }
  ll tot = 0;
  REP(dep, 0, 2 * n + 1) {
    VI &ei = tap[dep];
    ll cur = 0;
    REP(i, 0, ei.size()) {
      int a = i > 0 ? ei[i - 1] : -1;
      int b = ei[i];
      if (0) {
        DEBUGP(a);
        DEBUGP(b);
        DEBUGP(st.query(a,b));
      }
      if (i > 0 && st.query(a, b) < dep - n) {
        tot += cur * (cur - 1);
        cur = 1;
        continue;
      }
      cur++;
    }
    tot += cur * (cur - 1);
  }
  cout << tot / 2 << endl;
}
