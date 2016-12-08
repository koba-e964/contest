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

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;
const ll mod = 1e9 + 7;

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

struct PL_plus {
  PL operator()(PL x, PL y) const {
    return PL(x.first + y.first, x.second + y.second);
  }
};

PL PL_sub(PL x, PL y) {
  return PL(x.first - y.first, x.second - y.second);
}

int main(void){
  int n;
  scanf("%d", &n);
  vector<PI> pts(n);
  REP(i, 0, n) {
    int v, x;
    scanf("%d%d", &v, &x);
    pts[i] = PI(v, x);
  }
  sort(pts.begin(), pts.end());
  BIT<PL, PL_plus> bit(21000);
  ll tot = 0;
  REP(i, 0, n) {
    PI pt = pts[i];
    ll v = pt.first;
    ll x = pt.second;
    PL sum_left = bit.accum(x);
    PL sum_right = PL_sub(bit.accum(21000), bit.accum(x + 1));
    tot += v * (x * sum_left.second - sum_left.first);
    tot += v * (- x * sum_right.second + sum_right.first);
    bit.add(x + 1, PL(x, 1));
  }
  printf("%lld\n", tot);
}
