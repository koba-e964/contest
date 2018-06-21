#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

int main(void) {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  VI a(n), b(m);
  REP(i, 0, n) cin >> a[i];
  REP(i, 0, m) cin >> b[i];
  ll sum = 0;
  ll ma = 0;
  REP(i, 0, n) {
    ma = max(ma, (ll) a[i]);
  }
  int lo = n, hi = -1;
  REP(i, 0, n) {
    if (ma == a[i]) {
      lo = min(lo, i);
      hi = max(hi, i);
    }
  }
  int tap = 0;
  REP(i, 0, m) tap = max(tap, b[i]);
  sum += (ll) 1e9 * ma * (m - 1);
  REP(i, 0, n) sum += (ll) 1e9 * a[i];
  sum += b[0] * lo;
  REP(i, 0, m) sum += b[i];
  sum += tap * (hi - lo);
  sum += b[m - 1] * (n - hi - 1);
  cout << sum << "\n";
}
