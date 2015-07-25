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


ll powmod(ll x, ll e, ll mod) {
  ll sum = 1;
  ll cur = x;
  while (e > 0) {
    if (e % 2) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}

const ll Q = 0x10000;

// 0 <= solution <= lim
ll solve(ll x, ll p, ll s, int lim) {
  map<ll, ll> giant;
  REP(i, 0, lim / Q + 1) {
    giant.insert(pair<ll, ll>(powmod(x, i * Q, p), i * Q));
  }
  REP(i, 0, Q) {
    ll small = s * powmod(x, ((- i) % (p - 1)) + p - 1, p) % p;
    if (giant.count(small)) {
      ll d = giant[small];
      d += i;
      d %= p -1;
      if (d <= lim) {
	return d;
      }
    }
  }
  return -1;
}


int main(void){
  ll x, p;
  int a, b;
  cin >> x >> p >> a >> b;
  ll c = powmod(x, a, p);
  ll mi = c;
  if (b - a >= 0x1000000) { // discrete log
    int s = 1;
    while (1) {
      ll t = (ll)s * powmod(x, -(a % (p - 1)) + p - 1, p) % p;
      ll r = solve(x, p, t, b - a);
      if (r >= 0) {
	cout << s << endl;
	return 0;
      }
      s++;
    }
  }
  REP(i, 0, b - a) {
    c= c * x % p;
    mi=min(mi, c);
  }
  cout << mi << endl;
}
