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

ll gcd(ll x, ll y) {
  while (y != 0) {
    ll r = x % y;
    x = y; y = r;
  }
  return x;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  while (n--) {
    ll p, q, b;
    cin >> p >> q >> b;
    ll g = gcd(p, q);
    p /= g, q /= g;
    while (true) {
      ll g = gcd(q, b);
      if (g == 1) break;
      q /= g; b = g;
    }
    cout << (q == 1 ? "Finite" : "Infinite") << endl;
  }
}
