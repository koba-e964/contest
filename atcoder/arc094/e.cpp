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
  int n;
  cin >> n;
  VL a(n), b(n);
  ll tot = 0;
  ll mi = 1e10;
  bool all_eq = true;
  REP(i, 0, n) {
    cin >> a[i] >> b[i];
    tot += b[i];
    if (a[i] > b[i]) {
      mi = min(mi, b[i]);
    }
    if (a[i] != b[i]) {
      all_eq = false;
    }
  }
  cout << (all_eq ? 0 : tot - mi) << endl;
}
