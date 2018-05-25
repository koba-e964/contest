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
  VI a(n);
  REP(i, 0, n) cin >> a[i];
  int idx = n - 1;
  for (int i = n - 2; i >= 0; --i) {
    if (a[i] > a[i + 1]) {
      idx = i;
      break;
    }
  }
  bool ok = true;
  REP(i, 0, idx) {
    if (a[i] > a[i + 1]) ok = false;
  }
  if (idx < n - 1) {
    if (a[n - 1] > a[0]) ok = false;
  }
  cout << (ok ? n - 1 - idx : -1) << endl;
}
