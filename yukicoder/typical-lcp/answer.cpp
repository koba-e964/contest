#include <algorithm>
#include <bitset>
#include <cassert>
#include <iostream>
#include <string>
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
  std::vector<std::vector<int> > st;
  void create_sparse_table(int n, const std::vector<int> &lcp) {
    int h = 1;
    while ((1 << h) < n) {
      ++h;
    }
    st = std::vector<std::vector<int> >(h + 1, std::vector<int>(n));

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
  SparseTable(BiOp biop, const std::vector<int> &ary): biop(biop) {
    create_sparse_table(ary.size(), ary);
  }
  /*
   * Computes biop(ary[f], ary[f+1], ..., ary[s]). O(1).
   * Note: the interval is inclusive.
   */
  int query(int f, int s) const {
    assert (f <= s);
    int diff = top_bit(s + 1 - f);
    return biop(st[diff][f], st[diff][s + 1 - (1 << diff)]);
  }
};



const int N = 100010;
string s[N];

struct cmp {
  bool operator () (int x, int y) const {
    return s[x] < s[y];
  }
};

struct func_min {
  int operator() (int x, int y) const {
    return min(x, y);
  }
};

// Intended solution. This should be run as fast as possible.
int main(void){
  //ios::sync_with_stdio(false);
  //cin.tie(0);
  int n;
  cin >> n;
  assert (n <= 100000);
  int tot_len = 0;
  REP(i, 0, n) {
    cin >> s[i];
    tot_len += s[i].length();
  }
  assert (tot_len <= 800000);

  VI perm(n), inv_perm(n);
  REP(i, 0, n) { perm[i] = i; }
  sort(perm.begin(), perm.end(), cmp());
  REP(i, 0, n) { inv_perm[perm[i]] = i; }
  
  int m;
  cin >> m;
  assert (m <= 3000000);
  ll x, d;
  cin >> x >> d;
  ll lim = ll(n) * ll(n - 1);
  assert (0 <= x);
  assert (x < lim);
  assert (d < lim);
  assert (0 <= d);
  VI lcp(n - 1);
  REP(i, 0, n - 1) {
    int pos = 0;
    string &t1 = s[perm[i]];
    string &t2 = s[perm[i + 1]];
    int lim = min(t1.length(), t2.length());
    for (; pos < lim; ++pos) {
      if (t1[pos] != t2[pos]) { break; }
    }
    lcp[i] = pos;
  }
    
  SparseTable<int, func_min> st(func_min(), lcp);
  ll total = 0;
  REP(loop_cnt, 0, m) {
    int i, j;
    i = (x / (n - 1)) + 1;
    j = (x % (n - 1)) + 1;
    if (i > j) {
      swap(i, j);
    } else {
      j++;
    }
    assert (1 <= i);
    assert (i < j);
    assert (j <= n);
    i--, j--;
    int ii = inv_perm[i];
    int ij = inv_perm[j];
    if (ii > ij) {
      swap(ii, ij);
    }
    assert (ii < ij);
    total += st.query(ii, ij - 1);
    x = (x + d) % (ll(n) * ll(n - 1));
  }
  cout << total << "\n";
}
