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

ll gcd(ll x, ll y) {
  while (y != 0) {
    ll r = x % y;
    x = y;
    y = r;
  }
  return x;
}

const int N = 1000001;
ll fact[N];

ll invmod(ll x) {
  ll e = mod - 2;
  ll sum = 1;
  ll cur = x;
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
  res = fact[x] * invmod(res) % mod;
  return res;
}


int main(void){
  int h, w;
  cin >> h >> w;
  fact[0] = 1;
  REP(i, 1, N) {
    fact[i] = fact[i - 1] * i % mod;
  }
  int g = gcd(h, w);
  ll cnt = 0;
  REP(x, 0, g + 1) {
    int y = g - x;
    ll f1 = w / gcd(w, x);
    ll f2 = h / gcd(h, y);
    if (f1 / gcd(f1, f2) * f2 == h * ll(w) / g) {
      cnt = (cnt + comb(g, y)) % mod;
    }
  }
  cout << cnt << endl;
}
