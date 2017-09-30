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


const int DEBUG = 0;
const int TRIAL = 70;
// y = a[i] * x + b[i]


ll inversion(const vector<double> &v) {
  int n = v.size();
  set<double> vm;
  REP(i, 0, n) {
    vm.insert(v[i]);
  }
  vector<double> vm_v(vm.begin(), vm.end());
  sort(vm_v.begin(), vm_v.end());
  map<double, int> tbl;
  REP(i, 0, vm_v.size()) {
    tbl[vm_v[i]] = i;
  }
  BIT<int, plus<int> > bit(n + 1);
  ll tot = 0;
  REP(i, 0, n) {
    int val = tbl[v[i]] + 1;
    tot += i - bit.accum(val - 1);
    bit.add(val, 1);
  }
  return tot;
}

double binsect(const vector<pair<double, double> > &lines) {
  int n = lines.size();
  int rank = n * (n - 1) / 2;
  rank = 1 + rank / 2;
  if (DEBUG) {
    cerr << "n = " << n << ", rank = " << rank << endl;
  }
  // Find x s.t. {int | int >= x} = rank
  double lo = -1e10;
  double hi = 1e10;
  REP(puella, 0, TRIAL) {
      
    double mid = (hi + lo) / 2;
    vector<double> ary(n);
    REP(i, 0, n) {
      ary[i] = lines[i].first * mid + lines[i].second;
    }
    ll inv = inversion(ary);
    if (DEBUG) {
      cerr << "mid = " << mid << ", inv = " << inv << endl;
    }
    if (inv < rank) {
      hi = mid;
    } else {
      lo = mid;
    }
  }
  return lo;
}


vector<pair<double, double> > calc(const vector<double> &a,
				   const vector<double> &b,
				   const vector<double> &c) {
  int n = a.size();
  vector<pair<double, double> > ret;
  REP(i, 0, n) {
    double p = -a[i] / b[i];
    double q = c[i] / b[i];
    ret.push_back(make_pair(p, q));
  }
  sort(ret.begin(), ret.end());
  return ret;
}

int main(void) {
  int n;
  cin >> n;
  vector<double> a(n), b(n), c(n);
  REP(i, 0, n) {
    cin >> a[i] >> b[i] >> c[i];
  }
  vector<pair<double, double> > sx = calc(a, b, c);
  vector<pair<double, double> > sy = calc(b, a, c);
  double midx = binsect(sx);
  double midy = binsect(sy);
  #if 0
  // O(n^2 log(n))
  vector<double> px, py;
  REP(i, 0, n) {
    REP(j, i + 1, n) {
      double xint = (b[i] - b[j]) / (a[j] - a[i]);
      double yint = xint * a[i] + b[i];
      px.push_back(xint);
      py.push_back(yint);
    }
  }
  sort(px.begin(), px.end());
  sort(py.begin(), py.end());
  #endif
  printf("%.15f %.15f\n", midx, midy);
}
