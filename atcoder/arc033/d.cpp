#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;
const ll mod = 1e9 + 7;

const int DEBUG = 1;

const int N = 100010;
int a[N];

ll modpow(ll x, int n) {
  ll cur = x % mod;
  ll sum = 1;
  while (n > 0) {
    if (n % 2 != 0) {
      sum *= cur;
      sum %= mod;
    }
    cur *= cur;
    cur %= mod;
    n /= 2;
  }
  return sum;
}

ll invmod(int x) {
  int n = (int)mod - 2;
  return modpow(x,n);
}

ll divmod(ll x, ll y) {
  return x * invmod(y) % mod;
}

ll fact[N];
ll tbl1[N], tbl2[N];

int main(void){
  int n;
  ll t;
  cin >> n;
  REP(i, 0, n + 1) {
    cin >> a[i];
  }
  ll cur = 1;
  fact[0] = 1;
  REP(i, 1, N) {
    fact[i] = fact[i - 1] * i % mod;
  }
  cin >> t;
  /* \sum a[i] * (t - n) * (t - (n - 1)) * ... (t - (i + 1)) * (t - (i - 1)) * ... * (t - 0)
     /
     ((i - n) * (i - (n - 1)) * ... (i - (i + 1)) * (i - (i - 1)) * ... * (i - 0))
   */
  tbl1[0] = 1;
  tbl2[0] = 1;
  REP(i, 1, n + 1) {
    tbl1[i] = tbl1[i - 1] * (t - (n - i + 1) + mod) % mod;
    tbl2[i] = tbl2[i - 1] * (t - (i - 1) + mod) % mod;
  }
  ll sum = 0;
  REP(i, 0, n + 1) {
    ll tmp = fact[n - i];
    if ((n - i) % 2 == 1) {
      tmp = (mod - tmp) % mod;
    }
    tmp = tmp * fact[i] % mod;
    tmp = divmod(tbl1[n - i] * tbl2[i] % mod, tmp);
    tmp = a[i] * tmp % mod;
    sum += tmp;
    sum %= mod;
  }
  cout << sum << endl;
}
