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
  int n, q;
  cin >> n >> q;
  VL a(n);
  REP(i, 0, n) cin >> a[i];
  REP(i, 0, n) a[i] = 10 * a[i] + 1;
  VL x(q);
  REP(i, 0, q) cin >> x[i];
  REP(i, 0, q) x[i] *= 10;
  VL acc(n + 1);
  VL acc2(n + 1);
  REP(i, 0, n) acc[i + 1] = acc[i] + a[i] / 10;
  acc2[1] = a[0] / 10;
  REP(i, 1, n) acc2[i + 1] = acc2[i - 1] + a[i] / 10;
  REP(i, 0, q) {
    ll fail = 1e11;
    ll pass = -1;
    while (fail - pass > 1) {
      ll mid = (pass + fail) / 2;
      int idx0 = lower_bound(a.begin(), a.end(), x[i] - mid) - a.begin();
      int idx1 = upper_bound(a.begin(), a.end(), x[i] + mid) - a.begin();
      if (n - idx1 >= idx1 - idx0) pass = mid;
      else fail = mid;
    }
    int idx0 = lower_bound(a.begin(), a.end(), x[i] - pass) - a.begin();
    int idx1 = upper_bound(a.begin(), a.end(), x[i] + pass) - a.begin();
    ll tot = 0;
    assert ((n - idx1) - (idx1 - idx0) >= 0);
    assert ((n - idx1) - (idx1 - idx0) <= 1);
    if (n - idx1 == idx1 - idx0) {
      tot += acc[n] - acc[idx1] + acc2[idx0];
    } else {
      tot += acc[n] - acc[idx1] + (idx0 >= 1 ? acc2[idx0 - 1] : 0);
    }
    cout << tot << "\n";
  }
}
