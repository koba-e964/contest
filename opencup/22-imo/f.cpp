#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
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

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll p;
  int q;
  cin >> p >> q;
  REP(i, 0, q) {
    ll a, b, c, d;
    cin >> a >> b >> c >> d;
    if ((a + b) % p != (c + d) % p) {
      cout << "-1\n";
      continue;
    }
    ll x = (a + b) % p;
    ll invx = powmod(x, p - 2, p);
    a = a * invx % p;
    c = c * invx % p;
    // 2^i a - t = c (mod p), 0 <= t < 2^i
    int ans = 0;
    while (1) {
      ll d = (a - c + p) % p;
      if (d < 1LL << ans) {
        break;
      }
      a = (2 * a) % p;
      ans++;
    }
    cout << ans << "\n";
  }
}
