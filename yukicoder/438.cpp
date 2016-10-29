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
string err = "mourennaihasimasenn";


ll ex_gcd(ll x, ll y, ll &a, ll &b) {
  if (y == 0) {
    a = 1;
    b = 0;
    return x;
  }
  ll q = x / y;
  ll r = x % y;
  ll res = ex_gcd(y, r, a, b);
  ll tmp = a - q * b;
  a = b;
  b = tmp;
  return res;
}

// gcd(x, y) == 1
ll reduce(ll x, ll y, ll &a, ll &b) {
  if (x > y) {
    return reduce(y, x, b, a);
  }
  ll q = b / x;
  a += q * y;
  b -= q * x;
  ll mi = 1e16;
  ll mi_d = -10001;
  REP(d, -10000, 10000) {
    ll ad = a + y * d;
    ll bd = b - x * d;
    ll c = abs(ad) + abs(bd);
    c = 2 * c - 1;
    if (mi > c) {
      mi = c;
      mi_d = d;
    }
  }
  a += mi_d * y;
  b -= mi_d * x;
  return mi;
}

void print(ll a, ll b) {
  ll cost = 2 * (abs(a) + abs(b)) - 1;
  if (cost <= 10000) {
      // print
    if (a < 0) {
      REP(i, 0, -a) cout << 'c';
      cout << 'w';
      REP(i, 0, -a) cout << 'W';
      REP(i, 1, b) cout << "wC";
      cout << endl;
      return;
    }
    if (b < 0) {
      REP(i, 0, -b) cout << 'w';
      cout << 'c';
      REP(i, 0, -b) cout << 'W';
      REP(i, 1, a) cout << "cC";
      cout << endl;
      return;
    }
    if (a >= 0 && b >= 0 && a + b > 0) {
      REP(i, 0, a) cout << 'c';
      REP(i, 0, b) cout << 'w';
      REP(i, 1, a + b) cout << 'C';
      cout << endl;
      return;
    }
    cout << "ccW" << endl;
  } else {
    cout << err << endl;
  }
}
int main(void){
  ll x, y, z;
  cin >> x >> y >> z;
  ll a, b;
  ll g = ex_gcd(x, y, a, b);
  assert (x * a + y * b == g);
  if (x + y == 0) {
    cout << (z == 0 ? "c" : err) << endl;
    return 0;
  }
  if (z % g != 0) {
    cout << err << endl;
    return 0;
  }
  if (x * y == 0) {
    if (x == 0) {
      if (z == 0) { print(1, 0); return 0; }
      ll q = z / y;
      print(0, q);
      return 0;
    }
    if (z == 0) { print(0, 1); return 0; }
    ll q = z / x;
    print(q, 0);
    return 0;
  }
  x /= g, y /= g, z /= g;
  a *= z, b *= z;
  ll cost = reduce(x, y, a, b);
  print(a, b);
}
