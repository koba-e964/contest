#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long ll;

const int N = 501;
const ll mod = 998244353;
ll dp[N][N];
ll fac[N], invfac[N];

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

int main(void){
    int n;
    cin >> n;
    fac[0] = 1;
    REP(i, 1, N) fac[i] = fac[i - 1] * i % mod;
    invfac[N - 1] = powmod(fac[N - 1], mod - 2);
    for (int i = N - 2; i >= 0; i--) invfac[i] = invfac[i + 1] * (i + 1) % mod;
    dp[1][0] = 1;
    REP(i, 1, n) {
        REP(j, 0, i) {
            ll cur = dp[i][j];
            if (cur == 0) continue;
            ll p2 = powmod(2, j);
            REP(k, 1, i + 1 - j) {
                ll comb = fac[i - j] * invfac[k] % mod * invfac[i - j - k] % mod;
                (dp[i + 1][j + k] += cur * p2 % mod * comb) %= mod;
            }
            (dp[i + 1][j] += cur * (p2 - 1)) %= mod;
        }
    }
    cout << dp[n][n - 1] << "\n";
}
