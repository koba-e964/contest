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
typedef pair<int, int> PI;
const double EPS=1e-9;

bool ok(ll n, const vector<ll> &x, ll k) {
  int m = x.size();
  vector<ll> fwd(m);
  if (x[0] > k) {
    return 0;
  }
  if (x[m - 1] < n - k - 1) {
    return 0;
  }
  fwd[0] = max(0LL, k - 2 * x[0]);
  fwd[0] = max(fwd[0], (k - x[0]) / 2);
  REP(i, 0, m - 1) {
    ll d = x[i + 1] - x[i];
    if (d <= fwd[i] + 1) {
      fwd[i + 1] = k; // completely go right
    } else if (fwd[i] + k + 1 < d) {
      return 0; //impossible
    } else {
      ll tmp = max(0LL, k - 2 * (d - fwd[i] - 1));
      fwd[i + 1] = max(tmp, (k - (d - fwd[i] - 1)) / 2);
    }
  }
  return fwd[m - 1] >= n - 1 - x[m - 1];
}

int main(void){
  ll n;
  int m;
  cin >> n >> m;
  vector<ll> x(m);
  REP(i, 0, m) {
    cin >> x[i];
    x[i]--;
  }
  if (m == 1) {
    cout << min(n - 1 + x[0], 2 * (n - 1) - x[0]) << endl;
    return 0;
  }
  ll lo = -1, hi = 2 * n;
  // (lo, hi]
  while (hi - lo >= 2) {
    ll mid = (lo + hi) / 2;
    assert (mid >= 0);
    bool res = ok(n, x, mid);
    if (res) {
      hi = mid;
    } else {
      lo = mid;
    }
  }
  cout << hi << endl;
}
