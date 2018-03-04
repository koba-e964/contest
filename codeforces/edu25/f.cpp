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

/*
 * Suffix Array.
 * Required Headers: algorithm, cassert, string, vector
 * Verified by: http://arc050.contest.atcoder.jp/submissions/818745
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


/*
 * Z algorithm. Calculates an array a[i] = |lcp(s, s[i...|s|])|,
 * where s is the given string.
 * If n = s.length(), the returned array has length n + 1.
 * E.g. z_algorithm("ababa") = {5, 0, 3, 0, 1, 0}
 * Reference: http://snuke.hatenablog.com/entry/2014/12/03/214243
 * Header Requirement: vector, string
 * Verified by: AtCoder ARC055-C (http://arc055.contest.atcoder.jp/submissions/950597)
 */
std::vector<int> z_algorithm(const std::string &s) {
  int n = s.length();
  std::vector<int> ret(n + 1);
  ret[0] = n;
  for (int i = 1, j = 0; i < n;) {
    for (; i + j < n && s[j] == s[i + j]; ++j) {}
    ret[i] = j;
    if (j == 0) { ++i; continue; }
    int k = 1;
    for (; i + k < n && k + ret[k] < j; ++k) {
      ret[i + k] = ret[k];
    }
    i += k; j -= k;
  }
  return ret;
}


int num_len(int v) {
  int tot = 0;
  while (v > 0) {
    v /= 10;
    tot += 1;
  }
  return tot;
}

const int NON = 0;
const int N = 8100;
int dp[N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string s;
  if (NON) {
    int n;
    cin >> n;
    s = string(n, 'a');
  } else {
    cin >> s;
  }
  int n = s.length();
  VI sa = create_sa(s);
  VI inv(sa.size());
  REP(i, 0, sa.size()) inv[sa[i]] = i;
  VI lcp = create_lcp(s, sa);
  vector<VI> spt = create_sparse_table(n, lcp);

  dp[n] = 0;
  int globmi = n; // min of dp[i] + i
  for (int i = n - 1; i >= 0; --i) {
    int mi = 1 + n - i;
    mi = min(mi, globmi + 1 - i);
    REP(k, 1, (n - i) / 2 + 1) {
      int lcp = get_lcp(spt, inv[i], inv[i + k]);
      int ll = lcp / k;
      REP(l, 1, ll + 1) {
	int tmp = dp[i + k * (l + 1)] + num_len(l + 1) + k;
	mi = min(mi, tmp);
      }
    }
    dp[i] = mi;
    globmi = min(globmi, mi + i);
  }
  cout << dp[0] << "\n";
}
