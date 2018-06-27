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
  VL a(n);
  REP(i, 0, n) cin >> a[i];
  pair<ll, int> mi(1e18, -1);
  REP(i, 0, n) {
    mi = min(mi, make_pair((a[i] - i + n - 1 + n) / n * n - n + i, i));
  }
  cout << mi.second + 1 << "\n";
}
