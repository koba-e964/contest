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
  cin >> n;
  VL d(n);
  ll atot = 0;
  REP(i, 0, n) {
    ll a, b;
    cin >> a >> b;
    atot += a;
    d[i] = b + a;
  }
  sort(d.begin(), d.end());
  set<ll> former;
  set<ll> latter;
  REP(bits, 0, 1 << (n / 2)) {
    ll sum = 0;
    REP(i, 0, n / 2) {
      if (bits & 1 << i) {
	sum += d[i];
      }
    }
    former.insert(sum);
  }
  REP(bits, 0, 1 << (n - (n / 2))) {
    ll sum = 0;
    REP(i, 0, n - n / 2) {
      if (bits & 1 << i) {
	sum += d[i + n / 2];
      }
    }
    latter.insert(sum);
  }
  VL fv(former.begin(), former.end());
  VL lv(latter.begin(), latter.end());
  sort(fv.begin(), fv.end());
  sort(lv.begin(), lv.end());
  ll mi = 1e15;
  REP(i, 0, fv.size()) {
    ll t = fv[i];
    // We are to find position p s.t. abs(atot - t - lv[p]) is smallest.
    int lo = upper_bound(lv.begin(), lv.end(), atot - t)
      - lv.begin();
    int hi = lower_bound(lv.begin(), lv.end(), atot - t)
      - lv.begin();
    if (lo >= 1) {
      mi = min(mi, -(lv[lo - 1] + t) + atot);
    }
    if (hi < (int) lv.size()) {
      mi = min(mi, -atot + (lv[hi] + t));
    }
  }
  cout << mi << "\n";
}
