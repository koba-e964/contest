#include <algorithm>
#include <cassert>
#include <iostream>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef vector<int> VI;

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

int main(void){
  int k;
  string s;
  cin >> k >> s;
  if (k == 0) {
    cout << s << endl;
    return 0;
  }
  int n = s.length();
  VI sa = create_sa(s);
  VI sa_inv(n + 1);
  REP(i, 0, n + 1) {
    sa_inv[sa[i]] = i;
  }
  VI sat;
  int q = (n + k) / (k + 1);
  int r = q * (k + 1) - n; // 0 <= r <= k
  REP(i, 0, sa.size()) {
    int w = sa[i];
    if (w <= n - q) {
      sat.push_back(w);
    }
  }
  int lo = -1;
  int hi = n - q + 1;
  while (hi - lo > 1) {
    int mid = (hi + lo) / 2;
    // try to divide, so that all the divided strings <= s[sat[mid]..]
    // We want to avoid making more than r strings of length q - 1.
    int rem = r;
    REP(i, 0, k + 1) {
      if (rem < 0) {
	break;
      }
      int idx = i * q - r + rem;
      if (idx >= n - q + 1) {
	break;
      }
      if (sa_inv[idx] > sa_inv[sat[mid]]) {
	rem--;
      }
    }
    if (rem >= 0) {
      hi = mid;
    } else {
      lo = mid;
    }
  }
  cout << s.substr(sat[hi], q) << endl;
}
