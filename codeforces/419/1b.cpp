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

VL one_step(const VL &a) {
  // +-+-...+
  int n = a.size();
  assert (n % 2 == 1);
  VL ret(n - 1);
  REP(i, 0, n - 1) {
    if (i % 2 == 0) {
      ret[i] = (a[i] + a[i + 1]) % mod;
    } else {
      ret[i] = a[i] - a[i + 1];
      ret[i] = (ret[i] + mod) % mod;
    }
  }
  return ret;
}


int main(void){
  cin.tie(0);
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  ll sgn = 1;
  if (n == 1) {
    cout << a[0] % mod << endl;
    return 0;
  }
  if (n % 2 == 1) {
    a = one_step(a);
    n -= 1;
  }
  if (n % 4 == 0) {
    sgn = mod - 1;
  }
  ll x = 0;
  ll y = 0;
  int k = n / 2;
  ll cur = 1; // C(k - 1, i)
  REP(i, 0, k) {
    x = (x + cur * a[2 * i]) % mod;
    y = (y + cur * a[2 * i + 1]) % mod;
    cur = cur * (k - 1 - i) % mod;
    cur = cur * powmod(i + 1, mod - 2) % mod;
  }
  cout << (x + sgn * y) % mod << endl;
}
