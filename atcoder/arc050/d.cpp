#include <vector>
#include <cassert>
#include <string>
#include <algorithm>
/*
 * Suffix Array.
 * Required Headers: algorithm, cassert, string, vector
 * Verified by: 
 * Reference: http://mayokoex.hatenablog.com/entry/2016/04/03/145845
 */
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

std::vector<int> create_sa(const std::string& s) {
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

std::vector<int> create_lcp(const std::string &s, const std::vector<int> &sa) {
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

std::vector<std::vector<int> > create_sparse_table(int n, const std::vector<int> &lcp) {
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

int top_bit(int t) {
  for (int i = 31; i >= 0; --i) {
    if ((1 << i) & t) {
      return i;
    }
  }
  assert (0);
}

int get_lcp(const std::vector<std::vector<int> > &st, int f, int s) {
  if (f > s) {
    std::swap(f, s);
  }
  assert (f < s);
  int diff = top_bit(s - f);
  return std::min(st[diff][f], st[diff][s - (1 << diff)]);
}

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
typedef pair<int, int> PI;
const double EPS=1e-9;



struct cmp {
  const std::vector<int> &inv_sa;
  const std::vector<std::vector<int> > &st;
  cmp(const std::vector<int> &inv_sa, const std::vector<std::vector<int> > &st)
    : inv_sa(inv_sa), st(st) {}
  bool operator ()(int lhs, int rhs) {
    bool flip = 0;
    int n = inv_sa.size() - 1;
    if (lhs > rhs) {
      flip = 1;
      std::swap(lhs, rhs);
    }
    int len = get_lcp(st, inv_sa[lhs], inv_sa[rhs]);
    if (rhs + len < n) {
      return flip ^ (inv_sa[lhs] < inv_sa[rhs]);
    }
    return flip ^ (inv_sa[lhs + len] < inv_sa[lhs]);
  }
};

int main(void){
  int n;
  cin >> n;
  string s;
  cin >> s;
  VI sa = create_sa(s);
  VI lcp = create_lcp(s, sa);
  vector<VI> st = create_sparse_table(n, lcp);
  VI inv_sa(n + 1, -1);
  VI perm(n, -1);
  REP(i, 0, n + 1) {
    inv_sa[sa[i]] = i;
  }
  REP(i, 0, n) {
    perm[i] = i;
  }
  sort(perm.begin(), perm.end(), cmp(inv_sa, st));
  REP(i, 0, n) {
    cout << perm[i] + 1 << endl;
  }
}
