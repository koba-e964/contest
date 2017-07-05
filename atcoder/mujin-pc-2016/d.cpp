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


struct pmin {
  int operator()(int x, int y) const {
    return min(x, y);
  }
};

int main(void){
  int n, q;
  string s;
  cin >> n >> s >> q;
  VI acc_open(n + 1, 0), acc_close(n + 1, 0), acc_var(n + 1, 0),
    acc_neutral(n + 1, 0);
  REP(i, 0, n) {
    int o = s[i] == ')' ? -1 : 1;
    int cc = s[i] != '(' ? -1 : 1;
    int c = s[i] == '?' ? 1 : 0;
    int n = s[i] == '?' ? 0 : s[i] == '(' ? 1 : 0;
    acc_open[i + 1] = acc_open[i] + o;
    acc_close[i + 1] = acc_close[i] + cc;
    acc_var[i + 1] = acc_var[i] + c;
    acc_neutral[i + 1] = acc_neutral[i] + n;
  }
  SparseTable<int, pmin> spto(pmin(), acc_open);
  SparseTable<int, pmin> sptc(pmin(), acc_close);
  REP(test_nr, 0, q) {
    int l, r;
    cin >> l >> r;
    l--;
    if ((r - l) % 2 != 0) {
      cout << "No" << endl;
      continue;
    }
    // check [l, r)
    int half = (r - l) / 2;
    int diff = acc_neutral[r] - acc_neutral[l];
    int nvar = acc_var[r] - acc_var[l];
    if (half < diff || diff + nvar < half) {
      cout << "No" << endl;
      continue;
    }
    int no = half - diff; // #Opening parens
    int idx = lower_bound(acc_var.begin() + l, acc_var.begin() + r, acc_var[l] + no)
      - acc_var.begin();
    bool ok = true;
    if (l < idx && spto.query(l, idx) < acc_open[l]) {
      ok = false;
    }
    if (idx < r && sptc.query(idx, r) < acc_close[r]) {
      ok = false;
    }
    if (ok) {
      cout << "Yes" << endl;
    } else {
      cout << "No" << endl;
    }
  }
}
