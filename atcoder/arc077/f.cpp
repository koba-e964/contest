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


void experiment(string s) {
  REP(i, 0, 10) {
    int n = s.length();
    int v = -1;
    REP(i, (n + 2) / 2, n + 1) {
      if (s.substr(i) == s.substr(0, n - i)) {
	v = i;
	break;
      }
    }
    assert (v >= 0);
    s = s + s.substr(n - v, v - (n - v));
    cerr << "[" << n - v << ", " << v << ")" << endl;
    cerr << "len " << n << " -> " << 2 * v << endl;
  }
}
VL rec(const string &sa, const string &sb, VL &fib, VL &lfib, int v,
       ll x, ll &a, ll &b) {
  if (lfib[v] <= x) {
    a += fib[v];
    b += fib[v + 1];
    return VL(26, 0);
  }
  VL ret(26, 0);
  if (v <= 1) {
    const string &s = v == 0 ? sa : sb;
    REP(i, 0, x) {
      ret[s[i] - 'a'] += 1;
    }
    return ret;
  }
  if (x < lfib[v - 1]) {
    return rec(sa, sb, fib, lfib, v - 1, x, a, b);
  }
  rec(sa, sb, fib, lfib, v - 1, x, a, b); // no return
  return rec(sa, sb, fib, lfib, v - 2, x - lfib[v - 1], a, b);
}


VL calc(const string &s, ll x) {
  VI zarr = z_algorithm(s);
  int n = s.length();
  int k = n; // period
  REP(i, 1, n) {
    if (zarr[i] + i >= n) {
      k = i;
      break;
    }
  }
  if (n % k == 0) {
    VL ret(26, 0);
    REP(i, 0, k) {
      ll dup = x / k + (i < x % k ? 1 : 0);
      ret[s[i] - 'a'] += dup;
    }
    return ret;
  }
  ll a = 0, b = 0;
  VL fib(2, 0), lfib;
  {
    lfib.push_back(k);
    lfib.push_back(n);
    fib[0] = 1;
    while (true) {
      int len = fib.size();
      fib.push_back(fib[len - 2] + fib[len - 1]);
      lfib.push_back(lfib[len - 2] + lfib[len - 1]);
      if (lfib[len] >= x) {
	break;
      }
    }
    fib.push_back(fib[fib.size() - 2] + fib[fib.size() - 1]); // additional elem
  }
  VL ret = rec(s.substr(0, k), s.substr(0, n), fib, lfib, max(lfib.size() - 1, 1UL), x,
	       a, b);
  
  REP(i, 0, k) {
    ret[s[i] - 'a'] += a;
  }
  REP(i, 0, n) {
    ret[s[i] - 'a'] += b;
  }
  return ret;
}

int main(void){
  string s;
  cin >> s;
  ll l, r;
  cin >> l >> r;
  l--;
  {
    VI zarr = z_algorithm(s);
    int n = s.length();
    int v = n;
    REP(i, (s.length() + 2) / 2, s.length() + 1) {
      if (zarr[i] + i >= (int) s.length()) {
	v = i;
	break;
      }
    }
    s = s.substr(0, v);
  }
  VL t1 = calc(s, r);
  if (l >= 1) {
    VL t2 = calc(s, l);
    REP(i, 0, 26) {
      t1[i] -= t2[i];
    }
  }
  REP(i, 0, 26) {
    cout << t1[i] << (i == 25 ? "\n" : " ");
  }
}
