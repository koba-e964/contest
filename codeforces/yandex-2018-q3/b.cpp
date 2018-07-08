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

const ll inf = 1e8;

int main(void) {
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) cin >> a[i];
  VL cur(a);
  VL delta(n, 0);
  ll ans = 0;
  REP(i, 0, n - 1) {
    if (cur[i] > 0 && cur[i + 1] == 0) {
      cout << -1 << endl;
      return 0;
    }
    while (cur[i] > cur[i + 1]) {
      cur[i + 1] *= 2;
      ans++;
    }
    ans += delta[i];
    delta[i + 1] = delta[i];
    if (cur[i + 1] >= 2 * inf) {
      // Maneuver!
      cur[i + 1] /= 2;
      delta[i + 1] += 1;
    }
  }
  if (0) {
    cerr << "delta:";
    REP(i, 0, n) cerr << " " << delta[i];
    cerr << endl;
  }
  cout << ans << endl;
}
