#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <functional>
#include <iomanip>
#include <iostream>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

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

template<class T>
class TwoBIT {
  BIT<T, plus<T> > bit0, bit1;
public:
  TwoBIT(int n): bit0(n + 1), bit1(n + 1) {}
  void add_interval(int l, int r, T x) {
    bit1.add(l + 1, x);
    bit1.add(r + 1, -x);
    bit0.add(l + 1, -x * (l + 1));
    bit0.add(r + 1, x * (r + 1));
  }
  T sum_interval(int l, int r) {
    return bit1.accum(r + 1) * (r + 1) + bit0.accum(r + 1)
      - bit1.accum(l + 1) * (l + 1) - bit0.accum(l + 1);
  }
};

int main(void) {
  int n, m;
  cin >> n >> m;
  VI h(n);
  REP(i, 0, n) {
    cin >> h[i];
  }
  sort(h.rbegin(), h.rend());
  VI c(m);
  REP(i, 0, m) {
    cin >> c[i];
  }
  TwoBIT<ll> twobit(n);
  REP(i, 0, n) {
    twobit.add_interval(i, i + 1, h[i]);
  }
  int pos = 0;
  while (pos < m) {
    if (c[pos] > n) {
      break;
    }
    twobit.add_interval(0, c[pos], -1);
    int cc = twobit.sum_interval(c[pos] - 1, c[pos]);
    if (cc < 0) {
      break;
    }
    // sort
    if (c[pos] < n) {
      int cd = twobit.sum_interval(c[pos], c[pos] + 1);
      if (cc < cd) {
	int lo = -1;
	int hi = c[pos] - 1;
	while (hi - lo > 1) {
	  int mid = (hi + lo) / 2;
	  if (twobit.sum_interval(mid, c[pos])
	      == cc * (c[pos] - mid)) {
	    hi = mid;
	  } else {
	    lo = mid;
	  }
	}
	int start = hi;
	lo = c[pos];
	hi = n + 1;
	while (hi - lo > 1) {
	  int mid = (hi + lo) / 2;
	  if (twobit.sum_interval(c[pos], mid)
	      == cd * (mid - c[pos])) {
	    lo = mid;
	  } else {
	    hi = mid;
	  }
	}
	int end = lo;
	twobit.add_interval(start, start + (end - c[pos]), 1);
	twobit.add_interval(c[pos], end, -1);
      }
    }
    pos += 1;
    if (0) {
      cerr << "[" << pos - 1 << "]:";
      REP(i, 0, n) {
	cerr << " " << twobit.sum_interval(i, i + 1);
      }
      cerr << endl;
    }
  }
  cout << pos << endl;
}
