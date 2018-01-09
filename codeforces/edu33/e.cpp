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


const int N = 1100000;
int fp[N];
ll fact[2 * N];
ll invfact[2 * N];

void init(void) {
  REP(i, 2, N) {
    fp[i] = i;
  }
  REP(i, 2, 1100) {
    if (fp[i] == i) {
      for (int j = 2; j * i < N; ++j) {
	fp[i * j] = i;
      }
    }
  }
  fact[0] = 1;
  REP(i, 1, 2 * N) {
    fact[i] = fact[i - 1] * i % mod;
  }
  invfact[2 * N - 1] = powmod(fact[2 * N - 1], mod - 2);
  for (int i = 2 * N - 2; i >= 0; --i) {
    invfact[i] = invfact[i + 1] * (i + 1) % mod;
  }
}

ll g(int e, int y) {
  ll tmp = fact[e + y - 1];
  tmp = tmp * invfact[y - 1] % mod;
  return tmp * invfact[e] % mod;
}

ll f(int x, int y) {
  map<int, int> fact;
  while (x > 1) {
    int p = fp[x];
    fact[p] += 1;
    x /= p;
  }
  ll prod = 1;
  for (auto e: fact) {
    prod = prod * g(e.second, y) % mod;
  }
  prod = prod * powmod(2, y - 1) % mod;
  return prod;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int q;
  cin >> q;
  init();
  while (q--) {
    int x, y;
    cin >> x >> y;
    cout << f(x, y) << "\n";
  }
}
