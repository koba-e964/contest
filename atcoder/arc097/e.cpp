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

const int DEBUG = 0;

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

void chmin(int &x, int y) {
  x = min(x, y);
}


const int N = 2010;
int dp[N][N];
int delta_0[N][N], delta_1[N][N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI c(2 * n), a(2 * n);
  VI bp(n), wp(n);
  REP(i, 0, 2 * n) {
    char ch;
    cin >> ch >> a[i];
    c[i] = ch == 'B' ? 0 : 1;
    a[i]--;
    if (c[i] == 0) {
      bp[a[i]] = i;
    } else {
      wp[a[i]] = i;
    }
  }
  REP(i, 0, n + 1) REP(j, 0, n + 1) dp[i][j] = 1e8;
  dp[0][0] = 0;
  {
    SegTree<int, plus<int> > bl(2 * n, plus<int>(), 0);
    REP(i, 0, n + 1) {
      SegTree<int, plus<int> > wh(2 * n, plus<int>(), 0);
      REP(j, 0, n) {
	int val = wp[j];
	int sum = bl.query(0, val) + wh.query(0, val);
	delta_0[i][j] = val - sum;
	wh.update(val, 1);
      }
      if (i < n) {
	bl.update(bp[i], 1);
      }
    }
  }
  {
    SegTree<int, plus<int> > wh(2 * n, plus<int>(), 0);
    REP(i, 0, n + 1) {
      SegTree<int, plus<int> > bl(2 * n, plus<int>(), 0);
      REP(j, 0, n) {
	int val = bp[j];
	int sum = bl.query(0, val) + wh.query(0, val);
	delta_1[j][i] = val - sum;
	bl.update(val, 1);
      }
      if (i < n) {
	wh.update(wp[i], 1);
      }
    }
  }
  if (DEBUG) {
    cerr << "delta_0:" << endl;
    REP(i, 0, n) {
      REP(j, 0, n) {
	cerr << " " << delta_0[i][j];
      }
      cerr << endl;
    }
    cerr << endl;
    cerr << "delta_1:" << endl;
    REP(i, 0, n) {
      REP(j, 0, n) {
	cerr << " " << delta_1[i][j];
      }
      cerr << endl;
    }
    cerr << endl;
  }
  REP(i, 0, n + 1) {
    REP(j, 0, n + 1) {
      if (i < n) {
	chmin(dp[i + 1][j], dp[i][j] + delta_1[i][j]);
      }
      if (j < n) {
	chmin(dp[i][j + 1], dp[i][j] + delta_0[i][j]);
      }
    }
  }
  if (DEBUG) {
    cerr << "dp:" << endl;
    REP(i, 0, n + 1) {
      REP(j, 0, n + 1) cerr << " " << dp[i][j];
      cerr << endl;
    }
  }
  cout << dp[n][n] << endl;
}
