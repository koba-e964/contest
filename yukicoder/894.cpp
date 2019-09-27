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

ll gcd(ll x, ll y) {
  while (y != 0) {
    ll r = x % y;
    x = y; y = r;
  }
  return x;
}

ll c(ll a, ll b) {
  return (a + b - 1) / b;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll t, a, b;
  cin >> t >> a >> b;
  ll lcm = a / gcd(a, b);
  if ((double) lcm * (double) b > 1e18) {
    lcm = 2e18;
  } else {
    lcm *= b;
  }
  cout << c(t, a) + c(t, b) - c(t, lcm) << endl;
}
