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


const int N = 5124;
ll fact[N];
ll invfact[N];

void init(void) {
  fact[0] = 1;
  REP(i, 1, N) {
    fact[i] = fact[i - 1] * i % mod;
  }
  invfact[N - 1] = powmod(fact[N - 1], mod - 2);
  for (int i = N - 2; i >= 0; --i) {
    invfact[i] = invfact[i + 1] * (i + 1) % mod;
  }
}

ll comb(int x, int y) {
  if (y < 0 || y > x) {
    return 0;
  }
  return fact[x] * (invfact[y] * invfact[x - y] % mod) % mod;
}

ll partial_injection(int a, int b) {
  ll tot = 0;
  REP(x, 0, min(a, b) + 1) {
    tot += (comb(a, x) * comb(b, x) % mod) * fact[x] % mod;
    tot %= mod;
  }
  return tot;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  init();
  int a, b, c;
  cin >> a >> b >> c;
  ll prod = 1;
  prod = prod * partial_injection(a, b) % mod; 
  prod = prod * partial_injection(a, c) % mod; 
  prod = prod * partial_injection(c, b) % mod;
  cout << prod << "\n";
}
