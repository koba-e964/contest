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


struct omax {
  int operator()(int x, int y) const {
    return max(x, y);
  }
};

int main(void){
  int n, m, k;
  cin >> n >> m;
  vector<VI> a(n, VI(m));
  REP(i, 0, n) {
    REP(j, 0, m) {
      cin >> a[i][j];
    }
  }
  cin >> k;
  VI l(k), r(k);
  REP(i, 0, k) {
    cin >> l[i] >> r[i];
    l[i]--; r[i]--;
  }
  if (m <= 320) {
    // Sparse Table Sol
    vector<SparseTable<int, omax> > spt;
    REP(i, 0, m) {
      VI ary(n - 1);
      REP(j, 0, n - 1) {
	ary[j] = a[j][i] <= a[j + 1][i] ? 0 : 1;
      }
      spt.push_back(SparseTable<int, omax>(omax(), ary));
    }
    REP(i, 0, k) {
      bool ans = false;
      REP(j, 0, m) {
	if (l[i] >= r[i] || spt[j].query(l[i], r[i] - 1) == 0) {
	  ans = true;
	}
      }
      cout << (ans ? "Yes" : "No") << endl;
    }
  } else {
    // O(n^2 m)
    vector<vector<bool> > ans(n, vector<bool>(n));
    REP(i, 0, n) {
      vector<bool> tmp(m, true);
      REP(j, i, n) {
	bool ok = false;
	REP(k, 0, m) {
	  ok = ok || tmp[k];
	}
	ans[i][j] = ok;
	if (j < n - 1) {
	  REP(k, 0, m) {
	    tmp[k] = tmp[k] && a[j][k] <= a[j + 1][k];
	  }
	}
      }
    }
    REP(i, 0, k) {
      cout << (ans[l[i]][r[i]] ? "Yes" : "No") << endl;
    }
  }
}
