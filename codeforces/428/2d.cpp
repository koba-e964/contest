#include <algorithm>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
const ll mod = 1e9 + 7;

const int W = 1048575;
const int N = 200100;

ll pow2[N];
void init_pow2(void) {
  pow2[0] = 1;
  REP(i, 1, N) {
    pow2[i] = pow2[i - 1] * 2 % mod;
  }
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  init_pow2();
  int n;
  cin >> n;
  VI a(n);
  VI freq(W, 0);
  REP(i, 0, n) {
    cin >> a[i];
    freq[a[i]] += 1;
  }
  // find #clans s.t. x | gcd(...)
  VL dp(W);
  REP(x, 2, W) {
    int num = 0;
    for (int j = 1; j <= (W - 1) / x; ++j) {
      num += freq[x * j];
    }
    if (num >= 1) {
      dp[x] = pow2[num - 1] * num % mod;
    }
  }
  // inclusion-exclusion
  VL inex(W);
  for (int x = W - 1; x >= 2; --x) {
    ll tot = dp[x];
    for (int j = 2; j <= (W - 1) / x; ++j) {
      tot += mod - inex[x * j];
      tot %= mod;
    }
    inex[x] = tot;
  }
  ll tot = 0;
  REP(x, 2, W - 1) {
    tot += x * inex[x] % mod;
    tot %= mod;
  }
  cout << tot << endl;
}
