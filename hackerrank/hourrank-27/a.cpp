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
  int t;
  cin >> t;
  while (t--) {
    int n;
    cin >> n;
    VI a(n);
    REP(i, 0, n) cin >> a[i];
    bool ok = false;
    REP(i, 0, n) {
      VI t(a);
      t.erase(t.begin() + i);
      bool inc = true;
      REP(i, 0, n - 2) {
    if (t[i] > t[i + 1]) {
      inc = false;
    }
      }
      if (inc) {
    ok = true;
    break;
      }
    }
    cout << (ok ? "YES" : "NO") << endl;
  }
}
