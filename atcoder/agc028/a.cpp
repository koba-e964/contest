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

int gcd(int x, int y) {
  while (y != 0) {
    int r = x % y;
    x = y; y = r;
  }
  return x;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  string s, t;
  cin >> n >> m >> s >> t;
  int g = gcd(n, m);
  bool ok = 1;
  REP(i, 0, g) {
    if (s[n / g * i] != t[m / g * i]) {
      ok = false;
    }
  }
  if (not ok) {
    cout << -1 << endl;
  } else {
    cout << (ll) n / (ll) g * (ll) m << endl;
  }
}
