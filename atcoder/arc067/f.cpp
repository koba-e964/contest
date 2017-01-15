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
const ll mod = 1e9 + 7;

/*
 * Sparse Table.
 * BiOp should be the type of a binary operator which is
 * associative, commutative and idempotent.
 * (For example, min and gcd satisfy them.)
 * Header Requirement: vector, cassert
 * Verified by: AtCoder ARC023 D (http://arc023.contest.atcoder.jp/submissions/960757)
 */
template<class T, class BiOp>
class SparseTable {
private:
  BiOp biop;
  std::vector<std::vector<T> > st;
  void create_sparse_table(int n, const std::vector<T> &lcp) {
    int h = 1;
    while ((1 << h) < n) {
      ++h;
    }
    st = std::vector<std::vector<T> >(h + 1, std::vector<T>(n));

    for (int i = 0; i < n; ++i) {
      st[0][i] = lcp[i];
    }
    for (int j = 1; j <= h; ++j) {
      for (int i = 0; i <= n - (1 << j); ++i) {
	st[j][i] = biop(st[j - 1][i], st[j - 1][i + (1 << (j-1))]);
      }
    }
  }
  /*
   * Reference: https://graphics.stanford.edu/~seander/bithacks.html#IntegerLogFloat
   * Please be aware that it only works well in case of 1 <= t <= 2^24.
   */
  static int top_bit(int t) {
    const float v = t; // find int(log2(v)), where v > 0.0 && finite(v) && isnormal(v)
    int c;         // 32-bit int c gets the result;
    
    c = *(const int *) &v;  // OR, for portability:  memcpy(&c, &v, sizeof c);
    return (c >> 23) - 127;
  }
public:
  /*
   * Initializes this sparse table. O(n log n) where n = ary.size().
   */
  SparseTable(BiOp biop, const std::vector<T> &ary): biop(biop) {
    create_sparse_table(ary.size(), ary);
  }
  /*
   * Computes biop(ary[f], ary[f+1], ..., ary[s]). O(1).
   * Note: the interval is inclusive.
   */
  T query(int f, int s) const {
    assert (f <= s);
    int diff = top_bit(s + 1 - f);
    return biop(st[diff][f], st[diff][s + 1 - (1 << diff)]);
  }
};


const int N = 5010;
ll dp[N][N];

struct mmin {
  ll operator()(ll a, ll b) const {
    return max(a, b);
  }
};


int main(void){
  int n, m;
  cin >> n >> m;
  VL a(n - 1);
  REP(i, 0, n - 1) {
    cin >> a[i];
  }
  vector<VL> b(n, VL(m, 0));
  REP(i, 0, n) {
    REP(j, 0, m) {
      cin >> b[i][j];
    }
  }
  REP(j, 0, m) {
    VL tm(n);
    REP(i, 0, n) {
      tm[i] = b[i][j];
    }
    SparseTable<ll, mmin> st(mmin(), tm);
    REP(i, 0, n) {
      int lo = -1;
      int hi = i;
      while (hi - lo > 1) {
	int mid = (lo + hi) / 2;
	if (st.query(mid, i - 1) < tm[i]) {
	  hi = mid;
	} else {
	  lo = mid;
	}
      }
      int left = hi;
      int right = - 1;
      lo = i;
      hi = n;
      while (hi - lo > 1) {
	int mid = (lo + hi) / 2;
	if (st.query(i, mid) <= tm[i]) {
	  lo = mid;
	} else {
	  hi = mid;
	}
      }
      right = lo;
      if (0) {
	cerr << "(j, i, tm[i]) = " << j << ", " << i << ", " << tm[i];
	cerr << "(left,right) = " << left << ", " << right << endl;
      }
      // update [left, right] * [left, right]
      right++;
      dp[left][i] += tm[i];
      dp[left][right] -= tm[i];
      dp[i + 1][i] -= tm[i];
      dp[i + 1][right] += tm[i];
    }
  }
  REP(i, 0, n + 1) {
    REP(j, 0, n + 1) {
      dp[i][j] = dp[i][j] + (j >= 1 ? dp[i][j - 1] : 0) + (i >= 1 ? dp[i - 1][j] : 0) - (i >= 1 && j >= 1 ? dp[i - 1][j - 1] : 0);
    }
  }
  if (0) {
    REP(i, 0, n) {
      REP(j, 0, n) {
	cerr << "dp[" << i << "," << j << "]=" << dp[i][j] << endl;
      }
    }
  }
  ll ma = 0;
  REP(i, 0, n) {
    ll dist = 0;
    REP(j, i, n) {
      ma = max(ma, dp[i][j] - dist);
      if (j < n - 1) {
	dist += a[j];
      }
    }
  }
  cout << ma << endl;
}
