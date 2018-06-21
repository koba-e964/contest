#include <iostream>
#include <cassert>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const ll mod = 998244353;
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
void add(ll &x, ll y) {
  x = (x + y) % mod;
}

const int N = 3000;
ll fac[N], invfac[N];
void init(void) {
  fac[0] = 1;
  REP(i, 1, N) fac[i] = fac[i - 1] * i % mod;
  invfac[N - 1] = powmod(fac[N - 1], mod - 2);
  for (int i = N - 2; i >= 0; --i) invfac[i] = invfac[i + 1] * (i + 1) % mod;
}

ll naive(int n, int k) {
  assert (n <= 20);
  VL dp(1 << (n - 1));
  REP(bits, 0, 1 << (n - 1)) {
    int p = 1;
    VI tap;
    REP(i, 0, n - 1) {
      if (bits & 1 << i) {
	p++;
      } else {
	tap.push_back(p);
	p = 1;
      }
    }
    tap.push_back(p);
    ll yuki = 1;
    REP(i, 0, tap.size()) {
      yuki = yuki * invfac[tap[i]] % mod;
    }
    yuki = yuki * fac[n] % mod;
    dp[bits] = powmod(yuki, k);
    if (0) {
      cerr << "tap:";
      REP(i, 0, tap.size()) cerr << " " << tap[i];
      cerr << " => " << dp[bits] << endl;
      cerr << endl;
    }
  }
  for (int bits = (1 << (n - 1)) - 1; bits >= 0; --bits) {
    for (int sup = bits + 1; sup < 1 << (n - 1); ++sup) {
      if ((sup & bits) == bits) {
	add(dp[bits], mod - dp[sup]);
      }
    }
  }
  ll ans = dp[0];
  ans = ans * powmod(invfac[n], k - 1) % mod;
  return ans;
}

ll dp[N][2];

int main(void) {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  init();
  int n, k;
  cin >> n >> k;
  // ll ans = naive(n, k);
  dp[0][0] = 1;
  REP(i, 1, n + 1) {
    REP(j, 0, 2) {
      REP(l, 1, i + 1) {
        add(dp[i][j], dp[i - l][1 - j] * powmod(invfac[l], k));
      }
    }
  }
  ll ans = dp[n][0] - dp[n][1] + mod;
  ans %= mod;
  if (n % 2) ans = mod - ans;
  ans = ans * powmod(fac[n], 1) % mod;
  cout << ans << endl;
}
