#include <algorithm>
#include <cassert>
#include <cmath>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 2000, M = 30;
ll dp[N][M]; // dp[i][j]=|{ways to represent div[i] as a product of j numbers >= 2}|

const int T = 100010;
ll fact[T];

ll powmod(ll x, ll e) {
  ll sum = 1;
  ll cur = x % mod;
  while (e > 0) {
    if (e % 2) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}

ll comb(int x, int y) {
  ll res = fact[x - y] * fact[y] % mod;
  res = fact[x] * powmod(res, mod - 2) % mod;
  return res;
}

int main(void){
  int n;
  int m;
  cin >> n >> m;
  n = abs(n);
  fact[0] = 1;
  REP(i, 1, T) {
    fact[i] = fact[i - 1] * i % mod;
  }
  VI div;
  REP(i, 1, sqrt(n) + 1) {
    if (n % i == 0) {
      div.push_back(i);
      if (i * i != n) {
	div.push_back(n / i);
      }
    }
  }
  sort(div.begin(), div.end());
  assert (div.size() < N);
  dp[0][0] = 1;
  REP(i, 1, div.size()) {
    REP(k, 0, i) {
      if (div[i] % div[k] == 0) {
	REP(j, 1, M) {
	  dp[i][j] += dp[k][j - 1];
	  dp[i][j] %= mod;
	}
      }
    }
  }
  ll sum = 0;
  REP(i, 0, min(M, m + 1)) {
    ll tmp = dp[div.size() - 1][i] * powmod(2, m - 1) % mod;
    tmp *= comb(m, i);
    tmp %= mod;
    sum = (sum + tmp) % mod;
  }
  cout << sum << endl;
}
