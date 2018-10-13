#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>
#include <cassert>

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
    int a, b, x;
    cin >> a >> b >> x;
    if (x > (b + 1) / 2) {
      cout << -1 << endl;
      continue;
    }
    REP(i, 0, x) {
      cout << b - x + i + 1 << (i == x - 1 ? "\n" : " ");
    }
  }
}
