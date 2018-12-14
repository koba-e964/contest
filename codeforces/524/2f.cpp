#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;


void *supply(void);



/**
 * Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid I. Note that constructing this tree requires the identity
 * element of I and the operation of I.
 * Header requirement: vector, algorithm
 * Verified by ABC017-D (http://abc017.contest.atcoder.jp/submissions/660402)
 *             AGC009-C (http://agc009.contest.atcoder.jp/submissions/1461012)
 */

// Persistent Segment Tree
template<class I, class BiOp>
class SegTree {
  int n; // enclosing pow of 2
  int sz;
  SegTree *ch[2];
  I val;
public:
  SegTree() {}
  SegTree(int n_) {
    sz = n_;
    n = 1;
    while (n < n_) { n *= 2; } // n is a power of 2
    val = BiOp::e;
    if (n > 1) {
      ch[0] = new(supply()) SegTree(n / 2);
      ch[1] = new(supply()) SegTree(n_ - n / 2);
    }
  }
  SegTree(int n_, int encn) {
    sz = n_;
    n = encn;
    val = BiOp::e;
    if (n > 1) {
      ch[0] = new(supply()) SegTree(n / 2, n / 2);
      ch[1] = new(supply()) SegTree(n_ - n / 2);
    }
  }
  SegTree(int n_, int encn, int _nochild) {
    sz = n_;
    n = encn;
    val = BiOp::e;
  }
  void inplace_update(I v) {
    val = v;
    if (n > 1) {
      ch[0]->inplace_update(v);
      ch[1]->inplace_update(v);
    }
  }
  /* ary[k] <- v */
  SegTree *update(int k, I v) {
    assert (k < sz);
    assert (k >= 0);
    if (n <= 1) {
      SegTree *ret = new(supply()) SegTree(sz, n, 1);
      ret->val = v;
      return ret;
    }
    SegTree *ret = new(supply()) SegTree(sz, n, 1);
    if (k < n / 2) {
      ret->ch[0] = ch[0]->update(k, v);
      ret->ch[1] = ch[1];
    } else {
      ret->ch[0] = ch[0];
      ret->ch[1] = ch[1]->update(k - n / 2, v);
    }
    ret->val = BiOp::op(ret->ch[0]->val, ret->ch[1]->val);
    return ret;
  }
  /* [a, b) */
  I query(int a, int b) const {
    assert (0 <= a);
    assert (b <= sz);
    if (a >= b) return BiOp::e;
    if (a == 0 && b == sz) return val;
    int lo = a;
    int hi = min(b, n / 2);
    I left = ch[0]->query(lo, hi);
    lo = max(a, n / 2) - n / 2;
    hi = b - n / 2;
    I right = ch[1]->query(lo, hi);
    return BiOp::op(left, right);
  }
};


typedef pair<PI, int> PIPI;

struct pmin {
  static const int e = 1.1e9;
  static int op(int x, int y) {
    return min(x, y);
  }
};
SegTree<int, pmin> nodes[10000000];
int nodes_pos;
void *supply(void) {
  return &nodes[nodes_pos++];
}

int main(void) {
  int n, m, k;
  scanf("%d%d%d", &n, &m, &k);
  vector<PIPI> events;
  REP(i, 0, k) {
    int l, r, p;
    scanf("%d%d%d", &l, &r, &p);
    p--;
    events.push_back(PIPI(PI(r, l), p));
  }
  sort(events.begin(), events.end());
  vector<SegTree<int, pmin>*> chrono(events.size() + 1);
  chrono[0] = new(supply()) SegTree<int, pmin>(n);
  chrono[0]->inplace_update(-1e8);
  int pos = 0;
  for (auto &e: events) {
    // int r = e.first.first;
    int l = e.first.second;
    int p = e.second;
    int old = chrono[pos]->query(p, p + 1);
    chrono[pos + 1] = chrono[pos]->update(p, max(old, l));
    pos++;
  }
  REP(_, 0, m) {
    int a, b, x, y;
    scanf("%d%d%d%d", &a, &b, &x, &y);
    a--;
    int idx = upper_bound(events.begin(), events.end(), PIPI(PI(y, 2e9), 2e9))
      - events.begin();
    int res = chrono[idx]->query(a, b);
    if (res >= x) puts("yes");
    else puts("no");
    fflush(stdout);
  }
}
