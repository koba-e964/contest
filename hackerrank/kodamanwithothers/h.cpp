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

bool ok(const VL &a, ll x, ll y) {
  int n = a.size();
  int two = 0;
  REP(i, 0, n) {
    if (a[i] >= y + 2 * x + 1) {
      return false;
    }
    if (a[i] >= y + x + 1) {
      two += 1;
      if (two >= 2) return false;
      continue;
    }
    if (a[i] <= y) {
      two = 0;
    }
  }
  return true;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  ll x;
  cin >> n >> x;
  VL a(n);
  REP(i, 0, n) cin >> a[i];
  ll pass = 1e9, fail = -1;
  while (pass - fail > 1) {
    ll mid = (pass + fail) / 2;
    if (ok(a, x, mid)) {
      pass = mid;
    } else {
      fail = mid;
    }
  }
  cout << pass << endl;
}
