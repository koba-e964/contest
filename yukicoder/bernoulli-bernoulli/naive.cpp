#include <cassert>
#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const ll mod = 1e9 + 7;

ll powmod(ll a, ll e) {
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
  ll n;
  int k;
  cin >> n >> k;
  assert (1 <= n);
  assert (n <= (ll)1e16);
  assert (1 <= k);
  assert (k <= 10000);
  n %= mod;

  ll sum = 0;
  REP(i, 1, n + 1) {
    sum = (sum + powmod(i, k)) % mod;
  }
  cout << sum  << endl;
}
