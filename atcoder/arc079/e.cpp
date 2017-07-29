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

bool possum(VL a, ll x) {
  int n = a.size();
  REP(i, 0, n) {
    a[i] += x;
  }
  ll tt = 0;
  REP(i, 0, n) {
    tt += a[i] / (n + 1);
  }
  return tt >= x;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  ll lo = 0;
  ll hi = 51e16;
  while (hi - lo > 1) {
    ll mid = (hi + lo) / 2;
    if (possum(a, mid)) {
      lo = mid;
    } else {
      hi = mid;
    }
  }
  // Actual value is around lo
  ll cur = -1;
  for (ll x = max(lo - 2500, 0LL); x <= lo; x++) {
    if (possum(a, x)) {
      cur = x;
    } else {
      break;
    }
  }
  cout << cur << "\n";
}
