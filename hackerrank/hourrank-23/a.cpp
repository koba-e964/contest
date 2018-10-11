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
  int ans = 0;
  int p, d, m, s;
  cin >> p >> d >> m >> s;
  while (1) {
    if (s < p) break;
    s -= p;
    ans++;
    p = max(p - d, m);
  }
  cout << ans << endl;
}
