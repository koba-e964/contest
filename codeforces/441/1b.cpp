#include <cassert>
#include <iostream>
#include <functional>
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


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  BIT<int, plus<int> > bit(n);
  VI ans(n + 1);
  ans[0] = 1;
  REP(pos, 0, n) {
    int a;
    cin >> a;
    bit.add(a, 1);
    // check how many X's are in the rightmost block
    int lo = 0;
    int hi = n + 1;
    while (hi - lo > 1) {
      int mid = (hi + lo) / 2;
      int cnt = bit.accum(n) - bit.accum(mid - 1);
      if (cnt == n - mid + 1) {
	hi = mid;
      } else {
	lo = mid;
      }
    }
    ans[pos + 1] = pos + 2 - (n - hi + 1);
  }
  REP(i, 0, n + 1) {
    cout << ans[i] << (i == n ? "\n" : " ");
  }
}
