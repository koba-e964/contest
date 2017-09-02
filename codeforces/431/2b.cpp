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
  while (y != 0) {
    ll r = x % y;
    x = y;
    y = r;
  }
  return x;
}

struct quo {
  ll x, y;
  quo(ll x_, ll y_): x(x_), y(y_) {
    ll g = gcd(x, y);
    x /= g;
    y /= g;
    if (y < 0) {
      x = -x;
      y = -y;
    }
  }
  quo operator+(quo a) const {
    ll num = x * a.y + y * a.x;
    ll den = y * a.y;
    return quo(num, den);
  }
  quo operator-(quo a) const {
    ll num = x * a.y - y * a.x;
    ll den = y * a.y;
    return quo(num, den);
  }
  quo operator*(quo a) const {
    return quo(x * a.x, y * a.y);
  }
  bool operator <(quo a) const {
    return PI(x, y) < PI(a.x, a.y);
  }
};

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  set<quo> grad;
  REP(i, 1, n) {
    quo g(a[i] - a[0], i - 0);
    grad.insert(g);
  }
  REP(i, 0, n - 1) {
    quo g(a[n - 1] - a[i], (n - 1) - i);
    grad.insert(g);
  }
  for (quo g: grad) {
    set<quo> segm;
    REP(i, 0, n) {
      quo s = quo(a[i], 1) - g * quo(i, 1);
      segm.insert(s);
    }
    if (segm.size() == 2) {
      cout << "Yes" << endl;
      return 0;
    }
  }
  cout << "No" << endl;
}
