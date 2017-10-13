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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll inf = 5e18;

// 0 <= x < mod, 0 <= y, mod < 2^62
ll mulmod(ll x, ll y, ll mod) {
  ll sum = 0;
  ll cur = x;
  while (y > 0) {
    if (y % 2 == 1) {
      sum += cur;
      if (sum >= mod) {
	sum -= mod;
      }
    }
    cur *= 2;
    if (cur >= mod) {
      cur -= mod;
    }
    y /= 2;
  }
  return sum;
  
}

// 1 <= a < mod, 0 <= e, mod < 2^62
ll powmod(ll a, ll e, ll mod) {
  ll sum = 1;
  ll cur = a;
  while (e > 0) {
    if (e % 2) {
      sum = mulmod(sum, cur, mod);
    }
    cur = mulmod(cur, cur, mod);
    e /= 2;
  }
  return sum;
}

// x < 2^62
bool is_prime(ll x) {
  if (x <= 1) { return false; }
  if (x <= 3) { return true; }
  if (x % 2 == 0) {
    return false;
  }
  // From https://miller-rabin.appspot.com/
  ll bases[7] = {2, 325, 9375, 28178, 450775, 9780504, 1795265022};
  int e = 0;
  ll d = x - 1;
  while (d % 2 == 0) {
    e += 1;
    d /= 2;
  }
  REP(i, 0, 7) {
    ll r = bases[i] % x;
    if (r == 0) {
      continue;
    }
    ll t = powmod(r, d, x);
    bool ok = true;
    REP(j, 0, e) {
      if (t == 1) { break; }
      ll u = mulmod(t, t, x);
      if (u == 1 && t != x - 1) {
	ok = false;
	break;
      }
      t = u;
    }
    ok &= t == 1;
    if (not ok) {
      return false;
    }
  }
  return true;
}


ll pow_ll(ll x, int e) {
  assert (e >= 1);
  if (x <= 1) {
    return x;
  }
  ll cur = 1;
  ll lim = inf / x + 1;
  REP(_, 0, e) {
    if (cur >= lim) {
      return inf;
    }
    cur *= x;
  }
  return cur;
}


ll exact_root(ll x, int e) {
  if (e == 1) {
    return x;
  }
  ll y = pow(x, 1.0 / e);
  for (ll t = max(y - 10, 0LL); t <= y + 10; ++t) {
    if (pow_ll(t, e) == x) {
      return t;
    }
  }
  return -1;
}

bool is_prime_power(ll x) {
  assert (x < inf);
  bool is_square = exact_root(x, 2) != -1;
  for (int e = 1; e <= 45; e += is_square ? 1 : 2) {
    ll y = exact_root(x, e);
    if (y == -1) {
      continue;
    }
    if (is_prime(y)) {
      return true;
    }
  }
  return false;
}

bool solve(ll n) {
  if (n % 2 == 0) {
    return n >= 4;
  }
  // n is odd
  // n = 2^? + q^? (q >= 3)
  REP(b, 1, 62) {
    ll t = 1LL << b;
    if (n <= t) {
      break;
    }
    ll rest = n - t;
    if (is_prime_power(rest)) {
      return true;
    }
  }
  return false;
}

int main(void) {
  int q;
  cin >> q;
  while (q--) {
    ll n;
    cin >> n;
    cout << (solve(n) ? "Yes" : "No") << endl;
  }
}
