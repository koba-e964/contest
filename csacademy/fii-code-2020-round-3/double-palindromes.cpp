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

// https://snuke.hatenablog.com/entry/2014/12/02/235837
VI manacher(const string &S) {
  VI R(S.size());
  int i = 0, j = 0;
  while (i < (int)S.size()) {
    while (i-j >= 0 && i+j < S.size() && S[i-j] == S[i+j]) ++j;
    R[i] = j;
    int k = 1;
    while (i-k >= 0 && i+k < S.size() && k+R[i-k] < j) R[i+k] = R[i-k], ++k;
    i += k; j -= k;
  }
  return R;
}

struct pmax {
  int operator()(int a, int b) const {
    return a + b;
  }
};

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string s;
  cin >> s;
  int n = s.size();
  string dt(2 * n - 1, '*');
  REP(i, 0, n) {
    dt[2 * i] = s[i];
  }
  VI t = manacher(dt);
  SegTree<int, pmax> st(n - 1, pmax(), 0);
  map<int, VI> del;
  ll tot = 0;
  for (int i = n - 2; i >= 0; i--) {
    if (DEBUG) {
      cerr << "del[" << i << "]:";
      for (auto k: del) {
        cerr << k.first << ":";
        for (int u: k.second) cerr << " " << u;
        cerr << endl;
      }
      cerr << endl;
    }
    int rad = t[2 * i + 1] / 2;
    int sub = st.query(i, i + rad);
    if (DEBUG) {
      DEBUGP(sub);
    }
    tot += sub;
    int diff = rad - i;
    if (diff > -i) {
      del[diff].push_back(i);
      st.update(i, 1);
    }
    for (int v: del[-i]) {
      st.update(v, 0);
    }
    del.erase(-i);
  }
  cout << tot << endl;
}
