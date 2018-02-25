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

void add(ll &x, ll y) {
  x = (x + y) % mod;
}

void mul(ll &x, ll y) {
  x = x * y % mod;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  VI a(n), b(n);
  REP(i, 0, n) cin >> a[i];
  REP(i, 0, n) cin >> b[i];
  ll tot = 0;
  ll cum_eq = 1;
  ll invm = powmod(m, mod - 2);
  REP(i, 0, n) {
    // assume a and b differ at position i
    ll now = 0;
    ll new_cum = cum_eq;
    if (a[i] == 0 && b[i] == 0) {
      now = (m - 1) * (mod + 1) / 2 % mod;
      mul(now, invm);
      mul(new_cum, invm);
    } else if (a[i] == 0 && b[i] != 0) {
      now = m - b[i];
      mul(now, invm);
      mul(new_cum, invm);
    } else if (a[i] != 0 && b[i] == 0) {
      now = a[i] - 1;
      mul(now, invm);
      mul(new_cum, invm);
    } else {
      if (a[i] > b[i]) now = 1;
      else now = 0;
      if (a[i] != b[i]) {
	mul(new_cum, 0);
      }
    }
    add(tot, cum_eq * now);
    cum_eq = new_cum;
  }
  cout << tot << "\n";
}
