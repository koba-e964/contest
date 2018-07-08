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


const int N = 10003; // up to 10002
ll inv_tbl[N];
void calculate_inverse(void) {
  REP(i, 0, N) {
    inv_tbl[i] = powmod(i, mod - 2);
  }
}

// https://ja.wikipedia.org/wiki/%E3%83%99%E3%83%AB%E3%83%8C%E3%83%BC%E3%82%A4%E6%95%B0
ll ber[N];
void calculate_bernoulli(int k) {
  ber[0] = 1;
  REP(i, 1, k + 1) {
    ll sum = 0;
    ll cur = 1;
    REP(j, 0, i) {
      ll tmp = mod - cur;
      tmp = tmp * ber[j] % mod;
      sum = (sum + tmp) % mod;
      cur = cur * (i + 1 - j) % mod;
      cur = cur * inv_tbl[j + 1] % mod;
    }
    ber[i] = sum * inv_tbl[i + 1] % mod;
  }
}



int main(void) {
  ll n;
  int k;
  cin >> n >> k;
  assert (1 <= n);
  assert (n <= (ll)1e16);
  assert (1 <= k);
  assert (k <= 10000);
  n += 1;
  n %= mod;

  calculate_inverse();
  calculate_bernoulli(k);
  ll sum = 0;
  ll cur = n;
  ll comb = k + 1;
  REP(i, 0, k + 1) {
    ll tmp = cur * ber[k - i] % mod;
    tmp = tmp * comb % mod;
    sum = (sum + tmp) % mod;
    cur = cur * n % mod;
    comb = comb * (k - i) % mod;
    comb = comb * inv_tbl[i + 2] % mod;
  }
  cout << sum * inv_tbl[k + 1] % mod << endl;
}
