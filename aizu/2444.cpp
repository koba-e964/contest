#include <algorithm>
#include <cassert>
#include <iostream>
#include <string>
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



#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void){
  int n, m;
  string s;
  cin >> n >> m >> s;
  vector<int> sa = create_sa(s);
  VI lcp = create_lcp(s, sa);
  vector<VI> st = create_sparse_table(n, lcp);
  vector<int> sa_inv(n + 1);
  REP(i, 0, n + 1) {
    sa_inv[sa[i]] = i;
  }
  int l = 0, r = 1;
  vector<VI> cargo(n + 1);
  REP(i, 0, m) {
    string q;
    cin >> q;
    if (q == "L++") {
      l++;
    }
    if (q == "L--") {
      l--;
    }
    if (q == "R++") {
      r++;
    }
    if (q == "R--") {
      r--;
    }
    assert (l < r);
    cargo[r - l].push_back(sa_inv[l]);
  }
  int tot = 0;
  REP(i, 0, n + 1) {
    sort(cargo[i].begin(), cargo[i].end());
    // Counts how many distinct strings cargo[i] has
    REP(j, 0, cargo[i].size()) {
      bool ok = true;
      if (j >= 1) {
	int u = cargo[i][j - 1];
	int v = cargo[i][j];
	if (u == v || get_lcp(st, u, v) >= i) {
	  ok = false;
	}
      }
      tot += ok ? 1 : 0;
    }
  }
  cout << tot << endl;
}
