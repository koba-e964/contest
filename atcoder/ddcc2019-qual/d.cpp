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



ll gcd(ll x, ll y) {
  while (y != 0) {
    ll r = x % y;
    x = y; y = r;
  }
  return x;
}

void fail(void) {
  cout << "invalid" << endl;
  exit(0);
}

ll dig_sum(ll v, ll radix) {
  ll sum = 0;
  while (v > 0) {
    sum += v % radix;
    v /= radix;
  }
  return sum;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  VL a(31);
  REP(i, 2, 31) {
    cin >> a[i];
  }
  ll lcm = 1;
  ll val = 0;
  REP(i, 3, 31) {
    ll d = i - 1;
    ll r = a[i] % (i - 1);
    ll g = gcd(lcm, d);
    if (val % g != r % g) {
      fail();
    }
    ll nxtval = -1;
    REP(j, 0, d / g) {
      ll tmp = val + lcm * j;
      if (tmp % d == r) {
        nxtval = tmp;
        break;
      }
    }
    assert (nxtval >= 0);
    val = nxtval;
    lcm = lcm / g * d;
  }
  // verify
  if (val <= 0 || val > (ll) 1e12) {
    fail();
  }
  REP(i, 2, 31) {
    if (dig_sum(val, i) != a[i]) {
      fail();
    }
  }
  cout << val << endl;
}
