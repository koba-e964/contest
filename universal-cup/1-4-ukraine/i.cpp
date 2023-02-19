#include <algorithm>
#include <iomanip>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
const ll mod = 998244353;

ll calc(const vector<VI> &a) {
  int n = a.size();
  int m = a[0].size();
  VL dp(m + 1);
  VL acc(m + 1, 1);
  dp[m] = 1;
  REP(i, 0, n) {
    VL ep(m + 1, 0);
    int ma0 = 0;
    int mi1 = m;
    REP(j, 0, m) {
      int s = i + j + 1;
      if (a[i][j] != -1 && (a[i][j] < s || a[i][j] > s + 1)) {
        return 0;
      }
      if (a[i][j] != -1) {
        if (a[i][j] == s) {
          ma0 = max(ma0, j + 1);
        } else {
          mi1 = min(mi1, j);
        }
      }
    }
    if (ma0 > mi1) {
      return 0;
    }
    REP(j, ma0, mi1 + 1) {
      ep[j] = acc[j];
    }
    dp = ep;
    acc[m] = dp[m];
    for (int i = m - 1; i >= 0; i--) {
      acc[i] = acc[i + 1] + dp[i];
      if (acc[i] >= mod) {
        acc[i] -= mod;
      }
    }
  }
  return acc[0];
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  while (t--) {
    int n, m;
    cin >> n >> m;
    vector<VI> a(n, VI(m));
    REP(i, 0, n) {
      REP(j, 0, m) {
        cin >> a[i][j];
      }
    }
    cout << calc(a) << "\n";
  }
}
