#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

ll mod;

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

void add(ll &x, ll y) {
  x = (x + y) % mod;
}

const int N = 3010;
ll comb[N][N];
ll dp[N][N];
ll invfac[N];

void init(void) {
  comb[0][0] = 1;
  REP(i, 1, N) {
    REP(j, 0, N) {
      if (j > 0) add(comb[i][j], comb[i - 1][j - 1]);
      add(comb[i][j], comb[i - 1][j]);
    }
  }
  dp[0][0] = 1;
  REP(i, 1, N) {
    REP(j, 0, N) {
      if (j > 0) add(dp[i][j], dp[i - 1][j - 1]);
      add(dp[i][j], dp[i - 1][j]);
      dp[i][j] = dp[i][j] * j % mod;
      add(dp[i][j], dp[i - 1][j]);
    }
  }
  invfac[0] = 1;
  REP(i, 1, N) {
    invfac[i] = invfac[i - 1] * powmod(i, mod - 2) % mod;
  }
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n >> mod;
  init();
  ll tot = 0;
  REP(i, 0, n + 1) {
    ll fac = powmod(2, powmod(2, n - i, mod - 1));
    ll tis = powmod(2, n - i);
    ll ani = 0;
    ll accfac = 1;
    REP(j, 0, i + 1) {
      ll tmp = dp[i][j] * accfac % mod;
      accfac = accfac * tis % mod;
      tmp = tmp * invfac[j] % mod;
      add(ani, tmp);
    }
    ani = ani * fac % mod;
    ani = ani * comb[n][i] % mod;
    if (i % 2 == 1) ani = (mod - ani) % mod;
    add(tot, ani);
  }
  cout << tot << endl;
}
