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

ll n, a, b, c, d;

ll solve(ll p) {
  ll coef = 0;
  ll v = n;
  while (v > 0) {
    coef += v / p;
    v /= p;
  }
  return coef * (d + p * (c + p * (b + p * a)));
}

const int B = 1000000;
// const int B = 5;
int pr[B];
void init(void) {
  REP(i, 2, B) pr[i] = 1;
  for (int i = 2; i * i < B; ++i) {
    if (pr[i] == 0) continue;
    for (int j = 2; j * i < B; ++j) {
      pr[i * j] = 0;
    }
  }
}
int sieve[B];

int main(void) {
  init();
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n >> a >> b >> c >> d;
  ll ans = 0;
  VI primes;
  REP(i, 2, B) {
    if (pr[i]) {
      primes.push_back(i);
      ans += solve(i);
    }
  }
  for (int i = B; i <= n; i += B) {
    REP(j, 0, B) sieve[j] = 1;
    for (ll p: primes) {
      ll lo = (i + p - 1) / p * p;
      ll hi = (i + B - 1) / p * p;
      for (ll j = lo; j <= hi; j += p) {
	sieve[j - i] = 0;
      }
    }
    REP(j, 0, B) {
      if (sieve[j]) {
	ans += solve(i + j);
      }
    }
  }
  cout << (unsigned int) ans << endl;
}
