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
  int n, m, ds, cs, da, ca;
  cin >> n >> m >> ds >> cs >> da >> ca;
  VI h(n);
  REP(i, 0, n) cin >> h[i];
  int ma = 0;
  REP(aoe, 0, m + 1) {
    int rem = m - ca * aoe;
    if (rem < 0) continue;
    rem /= cs;
    VI sigma(h);
    REP(i, 0, n) sigma[i] -= da * aoe;
    sort(sigma.rbegin(), sigma.rend());
    int cnt = 0;
    while (not sigma.empty() && sigma.back() <= 0) {
      sigma.pop_back();
      cnt++;
    }
    reverse(sigma.begin(), sigma.end());
    REP(i, 0, sigma.size()) {
      int k = (sigma[i] + ds - 1) / ds;
      if (rem < k) break;
      rem -= k;
      cnt++;
    }
    ma = max(ma, cnt);
  }
  cout << ma << endl;
}
