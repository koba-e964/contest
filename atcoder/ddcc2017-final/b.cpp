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


ll gcd(ll x, ll y) {
  if (y == 0) {
    return x;
  }
  return gcd(y, x % y);
}

ll lcm(ll x, ll y) {
  ll g = gcd(x, y);
  return x / g * y;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  ll z;
  cin >> n >> z;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  ll l = 1;
  REP(i, 0, n) {
    l = lcm(l, gcd(z, a[i]));
  }
  cout << l << "\n";
}
