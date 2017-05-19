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
/**
 * For every prime factor p, perform result[p] += e. (where n = \prod p^e)
 * Verified by: Codeforces #400 E
 *              (http://codeforces.com/contest/776/submission/24956471)
 */
void factorize(long long v, std::map<long long, int> &result) {
  long long p = 2;
  while (v > 1 && p * p <= v) {
    int cnt = 0;
    while (v % p == 0) {
      cnt++;
      v /= p;
    }
    if (cnt > 0) {
      if (result.count(p) == 0) {
	result[p] = 0;
      }
      result[p] += cnt;
    }
    p += p == 2 ? 1 : 2;
  }
  if (v > 1) {
    if (result.count(v) == 0) {
      result[v] = 0;
    }
    result[v] += 1;
  }
}

ll gcd(ll x,ll y){
  while(y!=0){
    ll r=x%y;
    x=y;y=r;
  }
  return x;
}

// Find x s.t. n^x mod mod = 1, or -1 if there's no such x
ll calc(ll n, ll mod) {
  n %= mod;
  ll v = 1;
  REP(i, 0, mod) {
    v = v * n % mod;
    if (v == 1 % mod) {
      return i + 1;
    }
  }
  return -1;
}

int main(void){
  ll n;
  cin >> n;
  map<ll, int> factor;
  factorize(n, factor);
  ll res = 1;
  for (map<ll, int>::iterator it = factor.begin(); it != factor.end(); ++it) {
    ll p = it->first;
    int e = it->second;
    ll v = p - 1;
    REP(i, 0, e - 1) {
      v *= p;
    }
    ll c = calc(n, v);
    if (c == -1) {
      res = -1;
      break;
    }
    res = c / gcd(c, res) * res;
  }
  cout << res << endl;
}
