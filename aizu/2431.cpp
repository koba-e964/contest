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

/**
 * I should have typename I::value_type and
 * implement static I::value_type op(I::value_type, II::value_type).
 */
template<class Op>
class SegTree {
  int n;
  using I = typename Op::value_type;
  std::vector<I> dat;
public:
  SegTree(int n_) {
    n = 1;
    while (n < n_) { n *= 2; } // n is a power of 2
    dat.resize(2 * n);
    for (int i = 0; i < 2 * n - 1; i++) {
      dat[i] = Op::id;
    }
  }
  /* ary[k] <- v */
  void update(int k, I v) {
    k += n - 1;
    dat[k] = v;
    while (k > 0) {
      k = (k - 1) / 2;
      dat[k] = Op::op(dat[2 * k + 1], dat[2 * k + 2]);
    }
  }
  /* http://proc-cpuinfo.fixstars.com/2017/07/optimize-segment-tree/ */
  I query(int a, int b) const {
    I left = Op::id;
    I right = Op::id;
    a += n - 1;
    b += n - 1;
    while (a < b) {
      if ((a & 1) == 0) {
	left = Op::op(left, dat[a]);
      }
      if ((b & 1) == 0) {
	right = Op::op(dat[b - 1], right);
      }
      a = a / 2;
      b = (b - 1) / 2;
    }
    return Op::op(left, right);
  }
};

struct custom_ll {
  ll v;
  custom_ll(): v(0) {}
  custom_ll(ll v): v(v) {}
  operator ll() const { return v; }
  bool operator<(custom_ll o) {
    return v < o.v;
  }
};

struct max_ll {
  using value_type = custom_ll;
  static const custom_ll id;
  static custom_ll op(custom_ll x, custom_ll y) {
    return std::max(x, y);
  }
};
const custom_ll max_ll::id = custom_ll(std::numeric_limits<ll>::min());

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  scanf("%d", &n);
  std::vector<int> x(n);
  for (int i = 0; i < n; ++i)
    scanf("%d", &x[i]);
  auto seg =
    SegTree<max_ll>(n + 1);
  ll ma = 0;
  for (int i = 0; i < n; ++i) {
    ll res = std::max((ll)seg.query(1, x[i]), 0LL);
    ll cur = res + x[i];
    ma = std::max(ma, cur);
    seg.update(x[i], cur);
  }
  ll whole = (ll)n * (ll)(n + 1) / 2;
  printf("%lld\n", whole - ma);
}
