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
const ll mod = 1e9 + 7;
/*
 * Suffix Array.
 * Required Headers: algorithm, cassert, string, vector
 * Verified by: https://csacademy.com/submission/1443882/
 * Reference: http://mayokoex.hatenablog.com/entry/2016/04/03/145845
 */
class SuffixArray {
private:
  struct compare_sa {
    const std::vector<int> &rank;
    int n, k;
    compare_sa(const std::vector<int> &rank, int n, int k) : rank(rank), n(n), k(k) {}
    bool operator () (int i, int j) {
      if (rank[i] != rank[j]) {
	return rank[i] < rank[j];
      }
      int ri = i + k <= n ? rank[i + k] : -1;
      int rj = j + k <= n ? rank[j + k] : -1;
      return ri < rj;
    }
  };

  static std::vector<int> create_sa(const std::string& s) {
    int n = s.length();
    std::vector<int> sa(n + 1, -1);
    std::vector<int> rank(n + 1, -1);
    std::vector<int> tmp(n + 1, -1);
  
    for (int i = 0; i <= n; ++i) {
      sa[i] = i;
      rank[i] = i < n ? s[i] : -1;
    }
  
    for (int k = 1; k <= n; k *= 2) {
      compare_sa cp = compare_sa(rank, n, k);
      std::sort(sa.begin(), sa.begin() + n + 1, cp);
      tmp[sa[0]] = 0;
      for (int i = 1; i <= n; ++i) {
	tmp[sa[i]] = tmp[sa[i - 1]] + (cp(sa[i - 1], sa[i]) ? 1 : 0);
      }
      for (int i = 0; i <= n; ++i) {
	rank[i] = tmp[i];
      }
    }
    return sa;
  }

  static std::vector<int> create_lcp(const std::string &s, const std::vector<int> &sa) {
    int n = s.length();
    std::vector<int> rank(n + 1);
    std::vector<int> lcp(n, -1);
    for (int i = 0; i <= n; ++i) {
      rank[sa[i]] = i;
    }
    int h = 0;
    lcp[0] = 0;
    for (int i = 0; i < n; ++i) {
      int j = sa[rank[i] - 1];
      h = std::max(0, h - 1);
      for (; j + h < n && i + h < n; ++h) {
	if (s[j + h] != s[i + h]) {
	  break;
	}
      }
      lcp[rank[i] - 1] = h;
    }
    return lcp;
  }

  static std::vector<std::vector<int> > create_sparse_table(int n, const std::vector<int> &lcp) {
    int h = 1;
    while ((1 << h) < n) {
      ++h;
    }
    std::vector<std::vector<int> > st(h + 1, std::vector<int>(n));

    for (int i = 0; i < n; ++i) {
      st[0][i] = lcp[i];
    }
    for (int j = 1; j <= h; ++j) {
      for (int i = 0; i <= n - (1 << j); ++i) {
	st[j][i] = std::min(st[j - 1][i], st[j - 1][i + (1 << (j-1))]);
      }
    }
    return st;
  }

  static int top_bit(int t) {
    const double v = t;
    long long c;
    c = *(const long long *) &v;
    return (c >> 52) - 1023;
  }
  std::string s;
  std::vector<int> sa;
  std::vector<int> lcp;
  std::vector<std::vector<int> > spt;
public:
  SuffixArray(const std::string &s) : s(s) {
    sa = create_sa(s);
    lcp = create_lcp(s, sa);
    spt = create_sparse_table(s.length(), lcp);
  }
  int get_lcp(int f, int s) const {
    if (f > s) {
      std::swap(f, s);
    }
    assert (f < s);
    int diff = top_bit(s - f);
    return std::min(spt[diff][f], spt[diff][s - (1 << diff)]);
  }
  std::vector<int> suffix_array(void) const {
    return sa;
  }
  std::vector<int> inverse_array(void) const {
    std::vector<int> inv(sa.size());
    REP(i, 0, sa.size()) {
        inv[sa[i]] = i;
    }
    return inv;
  }
};



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string s;
  int k;
  cin >> s >> k;
  vector<string> pool;
  REP(i, 0, s.length()) {
    REP(j, 1, min(k, (int) s.length() - i) + 1) {
      string sub = s.substr(i, j);
      if (find(pool.begin(), pool.end(), sub) != pool.end()) continue;
      pool.push_back(sub);
      sort(pool.begin(), pool.end());
      while ((int) pool.size() > k) pool.pop_back();
    }
  }
  cout << pool.back() << endl;
}
