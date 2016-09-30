#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

ll invmod(ll x) {
  ll e = mod - 2;
  ll sum = 1;
  ll cur = x;
  while (e > 0) {
    if (e % 2 == 1) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}

const int N = 900100;
ll fact[N];
ll pow3[N];

ll comb(ll x, ll y) {
  ll ret = fact[x];
  ret *= invmod(fact[y]);
  ret %= mod;
  ret *= invmod(fact[x - y]);
  return ret % mod;
}

int main(void){
  int n, m, k;
  cin >> n >> m >> k;
  fact[0] = 1;
  pow3[0] = 1;
  REP(i, 1, N) {
    fact[i] = (fact[i - 1] * i) % mod;
    pow3[i] = pow3[i - 1] * 3 % mod;
  }
  
  // \sum_{b', c'} fact(b' + c' + n - 1) / fact(b') / fact(c') / fact(n - 1) * 3^(m - b' + k - c')
  // = \sum_{d = 0}^{m + k} fact(d + n - 1) / fact(n - 1) / fact(d) \sum_{b = max(0, d - k)}^{min(d, m)} C(d, b) 3^{m + k - d}
  ll sum = 0;
  ll cur = 1;
  REP(d, 0, m + k + 1) {
    ll tmp = fact[d + n - 1];
    tmp *= invmod(fact[n - 1]);
    tmp %= mod;
    tmp *= invmod(fact[d]);
    tmp %= mod;
    tmp *= pow3[m + k - d];
    tmp %= mod;
    tmp *= cur;
    sum += tmp;
    sum %= mod;
    cur *= 2;
    if (d >= k) {
      cur += mod - comb(d, d - k);
    }
    if (d >= m) {
      cur += mod - comb(d, m);
    }
    cur %= mod;
  }
  cout << sum << endl;
}
