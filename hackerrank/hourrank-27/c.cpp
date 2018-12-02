#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>
#include <cassert>
#include <map>

using namespace std;

const int DEBUG = 0;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;
typedef pair<PL, int> PPLI;

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

/**
 * Binary Indexed Tree (Fenwick Tree). Holds an array of type T.
 * T is a commutative monoid. Indices are 1 .. n.
 * Verified by AtCoder ARC043 C (http://arc043.contest.atcoder.jp/submissions/985157)
 */
template <class T, class Op> class BIT {
private:
  int n;
  std::vector<T> ary;
  Op op;
  T e;
public:
  BIT(int n, Op op = Op(), T e = T()) : op(op), e(e) {
    assert (n > 0);
    while(n != (n & (-n))) { // find the least power of 2 >= n
      n += n & (-n);
    }
    this->n = n;
    ary = std::vector<T>(n + 1);
    for(int i = 0; i <= n; i++) {
      ary[i] = e;
    }
  }
  /**
   * gets the sum in [1 .. idx]
   * @param idx
   * @return sum
   */
  T accum(int idx) {
    T sum = e;
    while(idx > 0) {
      sum = op(sum, ary[idx]);
      idx &= idx - 1;
    }
    return sum;
  }
  /**
   * performs data[idx] += val;
   */
  void add(int idx, T val) {
    assert (idx > 0);
    while(idx <= n) {
      ary[idx] = op(ary[idx], val);
      idx += idx & (-idx);
    }
  }
};


ll range(BIT<ll, plus<ll> > &x, int l, int r) {
  return x.accum(r + 1) - x.accum(l);
}

void solve(const VL &x, const VL &y, const VL &a, const VL &b, VL &ans) {
  int n = x.size();
  int q = a.size();
  vector<PPLI> pool;
  REP(i, 0, n) {
    pool.push_back(PPLI(PL(x[i], y[i]), 0));
  }
  REP(i, 0, q) {
    pool.push_back(PPLI(PL(a[i], b[i]), 1 + i));
  }
  VL yx(n + q);
  REP(i, 0, n + q) {
    PL v = pool[i].first;
    yx[i] = v.second - v.first;
  }
  sort(yx.begin(), yx.end());
  yx.erase(unique(yx.begin(), yx.end()), yx.end());
  map<ll, int> inv;
  int m = yx.size();
  REP(i, 0, m) inv[yx[i]] = i;
  BIT<ll, plus<ll> > st(m, plus<ll>(), 0);
  BIT<ll, plus<ll> > st2(m, plus<ll>(), 0);
  sort(pool.begin(), pool.end());
  ll curx = -1e18;
  ll cursum = 0;
  ll curcnt = 0;
  REP(i, 0, n + q) {
    PPLI cur = pool[i];
    ll u = cur.first.first, v = cur.first.second;
    int idx = inv[v - u];
    if (DEBUG) {
      cerr << u << " " << v << " " << "[ " << idx << "]" << " kind = " << cur.second << endl;
    }
    if (curx != u) {
      curx = u;
      cursum = 0;
      curcnt = 0;
    }
    if (cur.second == 0) {
      st.add(idx + 1, - v);
      st2.add(idx + 1, 1);
      cursum -= v;
      curcnt += 1;
    } else {
      int qidx = cur.second - 1;
      ll oldans = ans[qidx];
      ans[qidx] += 2 * range(st, 0, idx - 1);
      ans[qidx] += 2 * v * range(st2, 0, idx - 1);
      ans[qidx] += range(st, idx, idx);
      ans[qidx] += v * range(st2, idx, idx);
      // Counting points on x = u twice.
      ans[qidx] -= cursum;
      ans[qidx] -= v * curcnt;
      if (DEBUG) {
    cerr << "delta = " << ans[qidx] - oldans << endl;
      }
    }
    if (DEBUG) {
      REP(i, 0, m) cerr << " " << range(st, i, i);
      cerr << endl;
      REP(i, 0, m) cerr << " " << range(st2, i, i);
      cerr << endl;
    }
  }
}

int main(void) {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  int n, q;
  cin >> n >> q;
  VL x(n), y(n);
  REP(i, 0, n) {
    cin >> x[i] >> y[i];
  }
  VL a(q), b(q);
  REP(i, 0, q) {
    cin >> a[i] >> b[i];
  }
  VL ans(q);
  REP(flip, 0, 2) {
    REP(dir, 0, 4) {
      solve(x, y, a, b, ans);
      REP(i, 0, n) {
    ll tmp = x[i];
    x[i] = -y[i];
    y[i] = tmp;
      }
      REP(i, 0, q) {
    ll tmp = a[i];
    a[i] = -b[i];
    b[i] = tmp;
      }
    }
    REP(i, 0, n) {
      x[i] = -x[i];
    }
    REP(i, 0, q) {
      a[i] = -a[i];
    }
  }
  REP(i, 0, q) {
    cout << ans[i] / 2 << endl;
  }
}
