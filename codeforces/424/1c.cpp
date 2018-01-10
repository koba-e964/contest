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
  REP(j, 1, 320000) {
    fact.insert(j);
  }
  REP(i, 0, n) {
    REP(j, 1, 3200) {
      ll deck = (a[i] + j - 1) / j;
      if (deck >= 320000) {
	fact.insert(deck);
      }
    }
  }
  VL fv(fact.begin(), fact.end());
  fv.push_back(1e12);
  sort(fv.begin(), fv.end());
  int fvn = fv.size();
  ll ma = 1;
  for (int i = fvn - 2; i >= 0; --i) {
    ll lo = fv[i] - 1;
    ll hi = fv[i + 1];
    // check if fv[i] is okay
    {
      ll tot = 0;
      REP(j, 0, n) {
	tot += (fv[i] - (a[j] % fv[i])) % fv[i];
      }
      if (tot > k) {
	continue;
      }
    }
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
    ma = max(ma, lo);
    break;
  }
  cout << ma << "\n";
}
