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



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  ll k;
  cin >> n >> k;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  set<ll> fact;
  REP(j, 1, 32000) {
    fact.insert(j);
  }
  REP(i, 0, n) {
    REP(j, 1, 32000) {
      ll deck = (a[i] + j - 1) / j;
      if (deck >= 32000) {
	fact.insert(deck);
      }
    }
  }
  VL fv(fact.begin(), fact.end());
  fv.push_back(1e12);
  sort(fv.begin(), fv.end());
  int fvn = fv.size();
  ll ma = 1;
  REP(i, 0, fvn - 1) {
    ll lo = fv[i] - 1;
    ll hi = fv[i + 1];
    while (hi - lo > 1) {
      ll mid = (hi + lo) / 2;
      // check
      ll tot = 0;
      REP(j, 0, n) {
	tot += (mid - (a[j] % mid)) % mid;
      }
      if (tot <= k) {
	lo = mid;
      } else {
	hi = mid;
      }
    }
    if (lo == fv[i] - 1) {
      continue;
    }
    ma = max(ma, lo);
  }
  cout << ma << "\n";
}
