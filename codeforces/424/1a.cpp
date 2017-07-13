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

bool check(const VL &a, const VL &b, ll x) {
  int n = a.size();
  int k = b.size();
  vector<PI> range(n);
  REP(i, 0, n) {
    if (abs(a[i]) > x) {
      return false;
    }
    ll lo, hi;
    ll rem = (x - abs(a[i])) / 2;
    if (a[i] < 0) {
      lo = a[i] - rem;
      hi = rem;
    } else {
      lo = -rem;
      hi = a[i] + rem;
    }
    range[i] = PI(lower_bound(b.begin(), b.end(), lo) - b.begin(),
		  upper_bound(b.begin(), b.end(), hi) - b.begin());
  }
  // interval matching
  int pos = 0; // till pos matching was found (exclusive)
  REP(i, 0, n) {
    if (pos >= k) {
      return false;
    }
    PI r = range[i]; //[**, **)
    if (r.second <= pos) {
      return false;
    }
    if (r.second - r.first <= 0) {
      return false;
    }
    pos = max(pos, r.first) + 1;
  }
  return true;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k;
  ll p;
  cin >> n >> k >> p;
  VL a(n), b(k);
  REP(i, 0, n) {
    cin >> a[i];
    a[i] -= p;
  }
  REP(i, 0, k) {
    cin >> b[i];
    b[i] -= p;
  }
  sort(a.begin(), a.end());
  sort(b.begin(), b.end());
  ll lo = -1;
  ll hi = 1e15;
  while (hi - lo > 1) {
    ll mid = (hi + lo) / 2;
    if (check(a, b, mid)) {
      hi = mid;
    } else {
      lo = mid;
    }
  }
  cout << hi << "\n";
}
