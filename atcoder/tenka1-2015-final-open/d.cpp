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

ll exact_sqrt(ll x) {
  if (x < 0) {
    return -1;
  }
  if (x % 4 >= 2) {
    return -1;
  }
  ll y = sqrt(x);
  for (ll c = y - 1; c <= y + 1; ++c) {
    if (c * c == x) {
      return c;
    }
  }
  return -1;
}


ll gcd(ll x, ll y) {
  while (y != 0) {
    ll r = x % y;
    x = y;
    y = r;
  }
  return x;
}

int main(void) {
  ll l, s;
  cin >> l >> s;
  assert (l <= 10000);
  ll tot = 0;
  REP(c, 1, l) {
    REP(a, 1, c) {
      ll b = exact_sqrt(c * c - a * a - s * s);
      if (b >= a && b <= c && a + b + c <= l && c < a + b) {
	ll g = gcd(gcd(a, b), c);
	if (g == 1) {
	  tot += 1;
	}
      }
    }
  }
  cout << tot << endl;
}
