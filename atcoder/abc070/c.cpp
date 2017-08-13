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
  while (y != 0) {
    ll r = x % y;
    x = y;
    y = r;
  }
  return x;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL t(n);
  REP(i, 0, n) {
    cin >> t[i];
  }
  ll g = t[0];
  REP(i, 0, n) {
    g = g / gcd(g, t[i]) * t[i];
  }
  cout << g << endl;
}
