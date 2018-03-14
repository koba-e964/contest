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

struct pmax {
  int operator()(int x, int y) const {
    return max(x, y);
  }
};

SegTree<int, pmax> *create_seg(const string &s) {
  int n = s.length();
  SegTree<int, pmax> *ret = new SegTree<int, pmax>(n, pmax(), -1);
  REP(i, 0, n) {
    if (s[i] == 'B') {
      ret->update(i, i);
    }
  }
  return ret;
}

bool tra(int s, int t) {
  return s >= t && (s - t) % 3 == 0;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string s, t;
  cin >> s >> t;
  REP(i, 0, s.length()) {
    if (s[i] == 'C') s[i] = 'B';
  }
  REP(i, 0, t.length()) {
    if (t[i] == 'C') t[i] = 'B';
  }
  SegTree<int, pmax> *seg_s = create_seg(s);
  SegTree<int, pmax> *seg_t = create_seg(t);
  int slen = s.length();
  int tlen = t.length();
  VI sacc(slen + 1, 0), tacc(tlen + 1, 0);
  REP(i, 0, slen) {
    sacc[i + 1] = sacc[i] + (s[i] == 'B' ? 1 : 0);
  }
  REP(i, 0, tlen) {
    tacc[i + 1] = tacc[i] + (t[i] == 'B' ? 1 : 0);
  }
  int q;
  cin >> q;
  string ans(q, '+');
  REP(i, 0, q) {
    int a, b, c, d;
    cin >> a >> b >> c >> d;
    a--, c--;
    int bs = sacc[b] - sacc[a], bt = tacc[d] - tacc[c];
    if (bs > bt) {
      ans[i] = '0';
      continue;
    }
    int rs = seg_s->query(a, b - 1);
    int rt = seg_t->query(c, d - 1);
    int last_s = bs == 0 ? b - a : b - rs;
    int last_t = bt == 0 ? d - c : d - rt;
    if (bs == bt) {
      ans[i] = tra(last_s, last_t) ? '1' : '0';
      continue;
    }
    ans[i] = (bs + bt) % 2 == 0 && last_s >= last_t ? '1' : '0';
  }
  cout << ans << "\n";
}
