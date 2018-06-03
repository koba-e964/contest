#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
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


const int N = 300100;
ll fac[N], invfac[N];
void init(void) {
  fac[0] = 1;
  REP(i, 1, N) fac[i] = fac[i - 1] * i % mod;
  invfac[N - 1] = powmod(fac[N - 1], mod - 2);
  for (int i = N - 2; i >= 0; --i) invfac[i] = invfac[i + 1] * (i + 1) % mod;
}

int main(void) {
  init();
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll n, a, b, k;
  cin >> n >> a >> b >> k;
  ll tot = 0;
  REP(x, 0, n + 1) {
    ll rem = k - a * x;
    if (rem >= 0 && rem % b == 0) {
      ll y = rem / b;
      if (y <= n) {
        ll tmp = fac[n] * fac[n] % mod;
        tmp = tmp * (invfac[x] * invfac[n - x] % mod) % mod;
        tmp = tmp * (invfac[y] * invfac[n - y] % mod) % mod;
        add(tot, tmp);
      }
    }
  }
  cout << tot << endl;
}
