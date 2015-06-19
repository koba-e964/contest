#include <iostream>
#include <cstring>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

const ll mod = 1e9 + 7;
const int N = 600;

ll b2memo[N][N] = {};


ll b2(int n, int k) {
  ll &ret = b2memo[n][k];
  if (ret >= 0) {
    return ret;
  }
  if (n == 0 && k == 0) {
    return ret = 1;
  }
  if (n < k) {
    return ret = 0;
  }
  ll sub = k == 0 ? 0 : b2(n - 1, k - 1);
  sub += k * b2(n - 1, k);
  sub %= mod;
  return ret = sub;
}

ll cmemo[N][N];
ll comb(int n, int k) {
  ll &ret = cmemo[n][k];
  if (ret >= 0) {
    return ret;
  }
  if (n == 0 && k == 0) {
    return ret = 1;
  }
  if (n < k) {
    return ret = 0;
  }
  ll sub = k == 0 ? 0 : comb(n - 1, k - 1);
  sub += comb(n - 1, k);
  sub %= mod;
  return ret = sub;
}


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
  ll s = 0;
  memset(b2memo, -1, sizeof(b2memo));
  memset(cmemo, -1, sizeof(cmemo));
  REP(k, 1, n + 1) {
    int q = k * (k - 1);
    REP(l, k, n + 1) {
      ll sub = powmod(q, n - l);
      sub = sub * b2(l, k) % mod;
      sub = sub * comb(n, l) % mod;
      s = (s + sub) % mod;
    }
  }
  cout << s << endl;
}
