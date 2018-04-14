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
  VI a(3);
  REP(i, 0, 3) {
    cin >> a[i];
  }
  sort(a.begin(), a.end());
  int ans = 1e8;
  REP(i, 0, 2) {
    int lim = a[2] + i;
    int tot = 0;
    REP(j, 0, 3) {
      tot += lim - a[j];
    }
    if (tot % 2 == 0) {
      ans = tot / 2;
    }
  }
  cout << ans << endl;
}
