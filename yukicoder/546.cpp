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
  return y == 0 ? x : gcd(y, x % y);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  ll l, h;
  cin >> n >> l >> h;
  VL c(n);
  REP(i, 0, n) {
    cin >> c[i];
  }
  VL sanctus(1 << n);
  REP(bits, 1, 1 << n) {
    ll lcm = 1;
    REP(i, 0, n) {
      if (lcm == 0) {
	break;
      }
      if (bits & 1 << i) {
	ll g = gcd(lcm, c[i]);
	lcm = lcm / g * c[i];
	if (lcm > h) {
	  lcm = 0;
	}
      }
    }
    if (lcm == 0) {
      continue;
    }
    sanctus[bits] = h / lcm - (l - 1) / lcm;
  }
  for (int bits = (1 << n) - 1; bits >= 1; --bits) {
    for (int sup = bits + 1; sup < 1 << n; ++sup) {
      if ((sup & bits) != bits) { continue; }
      sanctus[bits] -= sanctus[sup];
    }
  }
  ll sum = 0;
  REP(i, 0, n) {
    sum += sanctus[1 << i];
  }
  cout << sum << "\n";
}
