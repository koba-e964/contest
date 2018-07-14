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

bool f(ll a, ll b, ll c, ll d) {
  if (b > d) return false;
  if (a < b) return false;
  ll g = gcd(b, d);
  ll r = (a - c) % g + g;
  r %= g;
  ll lim = c + r;
  if (r == 0) lim += g;
  return lim - b >= 0;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  while (t--) {
    ll a, b, c, d;
    cin >> a >> b >> c >> d;
    cout << (f(a, b, c, d) ? "Yes" : "No") << endl;
  }
}
