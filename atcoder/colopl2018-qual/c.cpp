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

ll gcd(ll x, ll y) {
  return y == 0 ? x : gcd(y, x % y);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll a, b;
  cin >> a >> b;
  int n = b - a + 1;
  VL d(n);
  REP(i, 0, n) {
    d[i] |= 1LL << i;
    REP(j, 0, n) {
      if (i == j) { continue; }
      if (gcd(a + i, a + j) == 1) {
	d[i] |= 1LL << j;
      }
    }
  }
  int m = n / 2;
  vector<PI> pool;
  REP(bits, 0, 1 << m) {
    ll cliq = (1LL << n) - 1;
    REP(i, 0, m) {
      if (bits & 1 << i) {
	cliq &= d[i];
      }
    }
    if ((bits & cliq) != bits) {
      continue;
    }
    int half = cliq >> m;
    pool.push_back(PI(bits, half));
  }
  VL snd(1 << (n - m), 0);
  REP(bits, 0, 1 << (n - m)) {
    ll cliq = (1LL << n) - 1;
    REP(i, 0, n - m) {
      if (bits & 1 << i) {
	cliq &= d[m + i];
      }
    }
    ll shift = (ll)bits << m;
    if ((shift & cliq) != shift) {
      continue;
    }
    snd[bits] = 1;
  }
  REP(i, 0, n - m) {
    REP(bits, 0, 1 << (n - m)) {
      if (bits & 1 << i) {
	snd[bits] += snd[bits ^ 1 << i];
      }
    }
  }
  ll tot = 0;
  REP(i, 0, pool.size()) {
    int half = pool[i].second;
    tot += snd[half];
  }
  cout << tot << "\n";
}
