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

/* http://d.hatena.ne.jp/komiyam/20120118/1326812613 */
std::vector<int> z_array(const std::string &str) {
  int len = str.length();
  std::vector<int> z(len);
  z[0] = len;
  for (int i = 1, left = 0, right = 0; i < len; ++i){
    if (i > right) {
      left = right = i;
      for(; right < len && str[right - left] == str[right]; ++right) {}
      z[i] = right - left;
      right--;
    }
    else{
      int k = i - left;
      if(z[k] < right - i + 1){
	z[i] = z[k];
      }
      else{
	left = i;
	for(;right < len && str[right - left] == str[right]; right++);
	z[i] = right - left;
	right--;
      }
    }
  }
  return z;
}

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 5000;
bool prime[1000000];
bool good[N][N + 1];
int dp[N + 1];



int main(void){
  string w;
  cin >> w;

  REP(i, 1, 1000000) {
    prime[i] = 1;
  }
  REP(i, 2, 1001) {
    if (! prime[i]) {
      continue;
    }
    REP(j, 2, 999999 / i + 1) {
      prime[i * j] = 0;
    }
  }
  assert (w.length() < N);
  int len = w.length();
  REP(st, 0, len) {
    VI za = z_array(w.substr(st));
    int n = w.length() - st;

    REP(i, 1, n + 1) {
      good[st][st + i] = 1;
    }
    REP(i, 1, n) {
      bool ok = 0;
      REP(j, 2, n / i + 1) {
	if (za[i * (j - 1)] < i) {
	  ok = 1;
	}
	good[st][st + i * j] &= ok;
      }
    }
  }
  dp[0] = 0;
  REP(i, 1, len + 1) {
    dp[i] = i;
    REP(j, 0, i) {
      if (good[j][i]) {
	dp[i] = min(dp[i], dp[j] + 1);
      }
    }
  }
  int mincnt = dp[len];
  cout << mincnt << endl;
  VL calc(len + 1, 0);
  calc[0] = 1;
  REP(i, 0, len) {
    REP(j, 0, i + 1) {
      if (good[j][i + 1] && dp[i + 1] - dp[j] == 1) {
	calc[i + 1] += calc[j];
	calc[i + 1] %= mod;
      }
    }
  }
  cout << calc[len] << endl;
}
