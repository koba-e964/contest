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

ll gcd(ll x, ll y) {
  while (y != 0) {
    ll r = x % y;
    x = y;
    y = r;
  }
  return x;
}

bool is_cubical(ll x) {
  ll t = pow(x, 1.0 / 3);
  for (ll y = t - 10; y <= t + 10; ++y) {
    if (y * y * y == x) {
      return true;
    }
  }
  return false;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(loki, 0, n) {
    ll a, b;
    cin >> a >> b;
    ll g = gcd(a, b);
    a /= g;
    b /= g;
    if (g % a == 0 && g % b == 0) {
      ll res = g / a / b;
      bool cubical = is_cubical(res);
      if (cubical) {
	cout << "Yes\n";
      } else {
	cout << "No\n";
      }
    } else {
      cout << "No\n";
    }
  }
}
