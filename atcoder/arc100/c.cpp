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

ll calc(const VL &a, ll x) {
  int n = a.size();
  ll tot = 0;
  REP(i, 0, n) {
    tot += abs(a[i] - x - i);
  }
  return tot;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cout << (1 | 2) << endl;
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) cin >> a[i];
  ll lo = -1e10, hi = 1e10;
  while (hi - lo > 3) {
    ll mid1 = (2 * lo + hi + (ll)3e11) / 3 - 1e11;
    ll mid2 = (lo + 2 * hi + (ll)3e11) / 3 - (ll)1e11;
    ll v1 = calc(a, mid1);
    ll v2 = calc(a, mid2);
    if (v1 >= v2) {
      lo = mid1;
    } else {
      hi = mid2;
    }
  }
  ll mi = 1e18;
  REP(i, -5, 5) {
    mi = min(mi, calc(a, lo + i));
  }
  cout << mi << endl;
}
