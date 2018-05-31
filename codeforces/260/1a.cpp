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

const int N = 100100;
ll dp[N];
ll cnt[N];

int main(void) {
  int n;
  cin >> n;
  VI a(n);
  REP(i, 0, n) cin >> a[i];
  REP(i, 0, n) cnt[a[i]] += 1;
  dp[1] = cnt[1];
  REP(i, 2, N) {
    ll ma = dp[i - 2];
    if (i >= 3) ma = max(ma, dp[i - 3]);
    dp[i] = ma + i * cnt[i];
  }
  cout << dp[N - 1] << endl;
}
