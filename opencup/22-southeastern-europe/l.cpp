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
#include <utility>
#include <vector>

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

using u64 = uint64_t;
using u128 = __uint128_t;

// https://cp-algorithms.com/algebra/primality_tests.html, modified
u64 binpower(u64 base, u64 e, u64 mod) {
    u64 result = 1;
    base %= mod;
    while (e) {
        if (e & 1)
            result = (u128)result * base % mod;
        base = (u128)base * base % mod;
        e >>= 1;
    }
    return result;
}

bool check_composite(u64 n, u64 a, u64 d, int s) {
    u64 x = binpower(a, d, n);
    if (x == 1 || x == n - 1)
        return false;
    for (int r = 1; r < s; r++) {
        x = (u128)x * x % n;
        if (x == n - 1)
            return false;
    }
    return true;
};

bool MillerRabin(u64 n) { // returns true if n is probably prime, else returns false.
    if (n < 4)
        return n == 2 || n == 3;
    if (n % 2 == 0 || n % 3 == 0) {
      return false;
    }

    // From https://miller-rabin.appspot.com/
    u64 bases[4] = { 2, 141889084524735, 1199124725622454117, 11096072698276303650ULL};

    int s = 0;
    u64 d = n - 1;
    while ((d & 1) == 0) {
        d >>= 1;
        s++;
    }

    for (int i = 0; i < 4; i++) {
        int a = bases[i] % n;
        if (a == 0) continue;
        if (check_composite(n, a, d, s))
            return false;
    }
    return true;
}

bool is_in(ll x, const VL &a) {
  for (ll b: a) x = min(x, x ^ b);
  return x == 0;
}

void add_vec(VL &a, ll x) {
  for (ll b: a) x = min(x, x ^ b);
  if (x == 0) return;
  a.push_back(x);
  sort(a.rbegin(), a.rend());
  int n = a.size();
  for (int i = n - 1; i >= 0; i--) {
    REP(j, 0, i) {
      a[j] = min(a[j], a[j] ^ a[i]);
    }
  }
}

ll nxt_not_in(ll x, const VL &a) {
  int cont = 0;
  int n = a.size();
  REP(i, 0, n) {
    if (a[n - 1 - i] == (1LL << cont)) {
      cont++;
    } else {
      break;
    }
  }
  ll val = 1LL << cont;
  if (x >= 4 && x % 2 == 0) x++;
  int tol = 10;
  while (is_in(x, a)) {
    x = (x - 1 & -val) + val + 1;
    if (x > 2 && x % 2 == 0) x++;
    tol--;
    if (tol == 0) return -1;
  }
  return x;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  mt19937_64 mt;
  while (t--) {
    ll l, r;
    cin >> l >> r;
    VL a;
    ll x = l;
    while (x <= r) {
      if (DEBUG) {
        cerr << "x = " << x;
      }
      ll nxt = nxt_not_in(x, a);
      if (DEBUG) {
        cerr << " nxt = " << nxt << endl;
      }
      if (nxt < 0) {
        break;
      }
      if (nxt <= r && MillerRabin(nxt)) {
        add_vec(a, nxt);
      }
      x = nxt + 1;
    }
    ll y = r;
    int tol = 100;
    while (y >= x) {
      if (MillerRabin(y)) {
        add_vec(a, y);
        tol = 100;
      } else {
        tol--;
        if (tol == 0) break;
      }
      y--;
    }
    if (x <= y) {
      ll len = y - x + 1;
      REP(_, 0, 10000) {
        ll val = x + (mt() % len);
        val |= 1;
        if (val <= r && MillerRabin(val)) {
          add_vec(a, val);
        }
      }
    }
    cout << (1LL << a.size()) << "\n";
  }
}
