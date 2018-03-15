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

/*
 * Suffix Array.
 * Required Headers: algorithm, cassert, string, vector
 * Verified by: https://csacademy.com/submission/1443859/
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


using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;



int main(void) {
    string a, b;
    cin >> a >> b;
    int n = a.length();
    string t = a + "#" + b;
    SuffixArray sa(t);
    VI sa_v = sa.suffix_array();
    VI inv = sa.inverse_array();
    VI acc(sa_v.size() + 1);
    REP(i, 0, sa_v.size()) {
        acc[i + 1] = acc[i] + (sa_v[i] >= n + 1 ? 1 : 0);
    }
    ll tot = 0;
    vector<PI> pool;
    REP(i, 0, n) {
        int idx = inv[i];
        int pass = n - i + 1;
        int fail = 0;
        while (pass - fail > 1) {
            int mid = (pass + fail) / 2;
            int hi = idx, lo = -1;
            while (hi - lo > 1) {
                int x = (hi + lo) / 2;
                if (sa.get_lcp(x, idx) >= mid) {
                    hi = x;
                } else {
                    lo = x;
                }
            }
            int left = hi;
            hi = sa_v.size(); lo = idx;
            while (hi - lo > 1) {
                int x = (hi + lo) / 2;
                if (sa.get_lcp(idx, x) >= mid) {
                    lo = x;
                } else {
                    hi = x;
                }
            }
            int right = lo;
            if (0) {
                DEBUGP(mid);
                DEBUGP(left);
                DEBUGP(right);
            }
            if (acc[right + 1] - acc[left] <= 0) {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        if (0) {
            DEBUGP(i);
            DEBUGP(pass);
        }
        pool.push_back(PI(idx, pass));
    }
    sort(pool.begin(), pool.end());
    REP(i, 0, pool.size()) {
        int idx = pool[i].first;
        int len = pool[i].second;
        tot += n + 1 - sa_v[idx] - len;
        if (i > 0) {
            int lcp = sa.get_lcp(pool[i - 1].first, idx);
            if (lcp >= len) {
                tot -= lcp - len + 1;
            }
        }
    }
    cout << tot << endl;
}
