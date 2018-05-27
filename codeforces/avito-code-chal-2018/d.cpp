#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k;
  cin >> n >> k;
  VL a(n);
  REP(i, 0, n) cin >> a[i];
  VL acc(n + 1);
  REP(i, 0, n) acc[i + 1] = acc[i] + a[i];
  ll ans = 0;
  for (int i = 56; i >= 0; --i) {
    vector<vector<bool> > dp(k + 1, vector<bool>(n + 1, false));
    dp[0][0] = true;
    ll sigma = ans | 1LL << i;
    REP(u, 0, k) {
      REP(j, 1, n + 1) {
        REP(l, 0, j) {
          if (not dp[u][l]) continue;
          if (((acc[j] - acc[l]) & sigma) != sigma) continue;
          dp[u + 1][j] = true;
        }
      }
    }
    if (dp[k][n]) ans = sigma;
  }
  cout << ans << endl;
}
