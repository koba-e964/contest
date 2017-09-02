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
typedef pair<ll, ll> PL;
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


int main(void) {
  int n;
  cin >> n;
  vector<PL> pt(n);
  REP(i, 0, n) {
    int x, y;
    cin >> x >> y;
    pt[i] = PL(x, y);
  }
  ll tot = powmod(2, n);
  tot += mod - 1 - n;
  tot %= mod;
  // Eliminate one-liners
  REP(i, 0, n) {
    REP(j, i + 1, n) {
      int elim = 0;
      REP(k, j + 1, n) {
	// i-j-k?
	PL v1(pt[j].first - pt[i].first, pt[j].second - pt[i].second);
	PL v2(pt[k].first - pt[i].first, pt[k].second - pt[i].second);
	ll outer = v1.first * v2.second - v1.second * v2.first;
	if (outer == 0) {
	  elim += 1;
	}
      }
      tot += mod - powmod(2, elim);
      tot %= mod;
    }
  }
  cout << tot << endl;
}
