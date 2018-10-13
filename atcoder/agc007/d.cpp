#include <algorithm>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<ll> VL;

template <class I, class BiOp> class SegmentTree {
  int n;
  std::vector<I> dat;
  BiOp op;
  I e;

public:
  typedef int size_type;
  typedef I value_type;
  SegmentTree(int n_, BiOp op, I e) : op(op), e(e) {
    n = 1;
    while (n < n_)
      n *= 2; // n is a power of 2
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
  /* http://proc-cpuinfo.fixstars.com/2017/07/optimize-segment-tree/
   * [a, b)
   */
  I query(int a, int b) const {
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
  I operator[](int idx) const {
    return dat[idx + n - 1];
  }
};


const ll inf = 1e16;

struct min_fun {
  ll operator()(ll x, ll y) const {
    return min(x, y);
  }
};

int main(void){
  int n;
  ll e, t;
  cin >> n >> e >> t;
  VL x(n);
  SegmentTree<ll, min_fun> raw(n + 1, min_fun(), inf); // dp[i]
  SegmentTree<ll, min_fun> boiled(n + 1, min_fun(), inf); // dp[i] - 2 * x[i]
  REP(i, 0, n) {
    cin >> x[i];
  }
  raw.update(0, 0);
  boiled.update(0, -2 * x[0]);
  REP(i, 0, n) {
    // Compute the range of j s.t. t <= 2 * (x[i] - x[j]) (0 <= j <= i)
    int j_boundary =
      upper_bound(x.begin(), x.begin() + i + 1, x[i] - (t+1)/2) - x.begin();
    ll ret = inf;
    ret = min(ret, boiled.query(0, j_boundary) + 2 * x[i]);
    ret = min(ret, raw.query(j_boundary, i + 1) + t);
    raw.update(i + 1, ret);
    if (i < n - 1) {
      boiled.update(i + 1, ret - 2 * x[i + 1]);
    }
  }
  cout << raw[n] + e << endl;
}
