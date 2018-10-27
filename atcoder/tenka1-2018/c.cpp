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



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) cin >> a[i];
  sort(a.begin(), a.end());
  ll tot = 0;
  if (n % 2 == 0) {
    REP(i, 0, n / 2 - 1) {
      tot -= 2 * a[i];
    }
    REP(i, n / 2 + 1, n) tot += 2 * a[i];
    tot -= a[n / 2 - 1];
    tot += a[n / 2];
  } else {
    REP(i, 0, n / 2 - 1) {
      tot -= 2 * a[i];
    }
    REP(i, n - (n / 2 - 1), n) {
      tot += 2 * a[i];
    }
    ll ma = - a[n / 2 - 1] - a[n / 2] + 2 * a[n / 2 + 1];
    ma = max(ma, -2 * a[n / 2 - 1] + a[n / 2] + a[n / 2 + 1]);
    tot += ma;
  }
  cout << tot << endl;
}
