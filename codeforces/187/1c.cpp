#include <algorithm>
#include <bitset>
#include <cassert>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
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


struct padd {
  ll operator()(ll x, ll y) {
    return (x + y) % mod;
  }
};

typedef pair<ll, int> PLI;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  vector<PI> a(n);
  REP(i, 0, n) {
    cin >> a[i].first;
    a[i].second = i;
  }
  sort(a.begin(), a.end());
  BIT<ll, padd> bit(n + 1, padd(), 0);
  ll ans = 0;
  REP(i, 0, n) {
    int idx = a[i].second;
    ll res = 0;
    res = bit.accum(idx);
    res = (res + 1) * (ll) a[i].first % mod;
    if (i > 0 && a[i].first == a[i - 1].first) {
      ll diff = 0;
      int prev = a[i - 1].second;
      diff = (bit.accum(prev) + 1) * (ll) a[i].first % mod;
      res = (res - diff + 2 * mod) % mod;
    }
    // Manipulate bit[idx + 1]
    bit.add(idx + 1, res);
    // cerr << idx << " " << res << " " << ans << endl;
    ans = (ans + res) % mod;
  }
  cout << ans << "\n";
}
