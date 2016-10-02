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

const int M = 5010;
string c[M];

// s + idx ??? q
int cmp(const string &s, int idx, const string &q) {
  int m = q.length();
  REP(i, 0, m) {
    if (idx + i >= s.length()) {
      return -1;
    }
    if (s[idx + i] != q[i]) {
      return s[idx + i] < q[i] ? -1 : 1;
    }
  }
  return 0;
}

int main(void){
  string s;
  int m;
  cin >> s >> m;
  REP(i, 0, m) {
    cin >> c[i];
  }
  VI sa = create_sa(s);
  int n = s.length();
  int sum = 0;
  REP(i, 0, m) {
    int lower, upper;
    // search c[i] by binary search
    int lo = 0;
    int hi = n + 1;
    while (hi - lo > 1) {
      int mid = (hi + lo) / 2;
      if (cmp(s, sa[mid], c[i]) < 0) {
	lo = mid;
      } else {
	hi = mid;
      }
    }
    lower = hi;
    lo = 0;
    hi = n + 1;
    while (hi - lo > 1) {
      int mid = (hi + lo) / 2;
      if (cmp(s, sa[mid], c[i]) > 0) {
	hi = mid;
      } else {
	lo = mid;
      }
    }
    upper = hi;
    sum += upper - lower;
  }
  cout << sum << endl;
}
