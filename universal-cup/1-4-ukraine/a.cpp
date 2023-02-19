#include <algorithm>
#include <iomanip>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<ll> VL;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  REP(_, 0, t) {
    int n;
    cin >> n;
    VL a(n);
    REP(i, 0, n) {
      cin >> a[i];
    }
    sort(a.begin(), a.end());
    VL b(n);
    REP(i, 0, (n + 1) / 2) {
      b[i] = a[2 * i];
    }
    REP(i, 0, n / 2) {
      b[i + (n + 1) / 2] = a[2 * (n / 2 - i - 1) + 1];
    }
    ll ans = b[0] * b[n - 1];
    REP(i, 0, n - 1) {
      ans += b[i] * b[i + 1];
    }
    cout << ans << "\n";
  }
}
