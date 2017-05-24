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


std::vector<int> perm_inv(const std::vector<int> &p) {
  int len = p.size();
  std::vector<int> ans(len);
  for (int i = 0; i < len; ++i) {
    assert (0 <= p[i] && p[i] < len);
    ans[p[i]] = i;
  }
  return ans;
}

// q after p
std::vector<int> perm_comp(const std::vector<int> &q, const std::vector<int> &p) {
  int n = p.size();
  std::vector<int> ret(n);
  for (int i = 0; i < n; ++i) {
    ret[i] = q[p[i]];
  }
  return ret;
}

ll powmod(ll a, ll e, ll mod) {
  ll sum = 1;
  ll cur = a;
  while (e > 0) {
    if (e % 2) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}


const int N = 0xe869120 / 2000;
const ll mods[2] = {ll(1e9 + 7), ll(1e9 + 9)};
ll hsh[N][2];

const ll coef = 41;
void hash_init(const string &s) {
  int n = s.length();
  hsh[n][0] = 0;
  hsh[n][1] = 0;
  for (int i = n - 1; i >= 0; --i) {
    REP(b, 0, 2) {
      ll h = coef * hsh[i + 1][b] + s[i];
      hsh[i][b] = h % mods[b];
    }
  }
}

vector<ll> get_hash(int a, int b) {
  vector<ll> ret(2);
  REP(i, 0, 2) {
    ll h1 = hsh[a][i];
    ll h2 = hsh[b][i];
    ll h = h1 + mods[i] - powmod(coef, b - a, mods[i]) * h2 % mods[i];
    h %= mods[i];
    ret[i] = h;
  }
  return ret;
}

int main(void){
  string s;
  int q;
  cin >> s >> q;
  VI sa = create_sa(s);
  VI sa_inv = perm_inv(sa);
  VI lcp = create_lcp(s, sa);
  vector<VI> spt = create_sparse_table(s.length(), lcp);

  // Rolling hash
  hash_init(s);
  
  REP(iter, 0, q) {
    int l, r, t;
    cin >> l >> r >> t;
    l--;
    int lcp = get_lcp(spt, sa_inv[l], sa_inv[l + t]);
    if (lcp >= r - l - t) {
      cout << "Yes" << endl;
      continue;
    }
    bool ok = false;
    VL fst = get_hash(l, r - t);
    VL snd = get_hash(l + t, r);
    vector<ll> f1s(2), f2s(2);
    REP(b, 0, 2) {
      f1s[b] = powmod(coef, lcp, mods[b]);
      f2s[b] = lcp >= t ? powmod(coef, lcp - t, mods[b]) : 0;
    }
    REP(c, 'a', 'z' + 1) {
      bool eq = true;
      REP(b, 0, 2) {
	ll diff = mods[b] + (ll) c - (ll) s[l + lcp];
	diff %= mods[b];
	ll f1 = f1s[b];
	ll f2 = f2s[b];
	if (((fst[b] + diff * f1 % mods[b]) - (snd[b] + diff * f2 % mods[b])) % mods[b] != 0) {
	  eq = false;
	  break;
	}
      }
      if (eq) {
	ok = true;
        break;
      }
    }
    REP(b, 0, 2) {
      f1s[b] = lcp + t >= r - t - l ? 0 : powmod(coef, lcp + t, mods[b]);;
      f2s[b] = powmod(coef, lcp, mods[b]);
    }
    REP(c, 'a', 'z' + 1) {
      if (ok) { break; }
      bool eq = true;
      REP(b, 0, 2) {
	ll diff = mods[b] + (ll) c - (ll) s[l + t + lcp];
	diff %= mods[b];
	ll f1 = f1s[b];
	ll f2 = f2s[b];
	if (((fst[b] + diff * f1 % mods[b]) - (snd[b] + diff * f2 % mods[b])) % mods[b] != 0) {
	  eq = false;
	  break;
	}
      }
      if (eq) {
	ok = true;
        break;
      }
    }
    if (ok) {
      cout << "Yes" << endl;
      continue;
    }
    cout << "No" << endl;
  }
}
