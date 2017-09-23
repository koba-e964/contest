#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;

int log2_int(ll x) {
  int cur = 0;
  while (true) {
    if (x < 1LL << cur) {
      return cur - 1;
    }
    cur += 1;
  }
}

// This solution was implemented after the author read the editorial.
ll solve(const VL &a) {
  int n = a.size();
  VI b(n + 2), c(n + 2);
  REP(i, 0, n) {
    b[i + 1] = log2_int(a[i]);
    c[i + 1] = log2_int(a[i] - 1) + 1;
  }
  vector<VI> dp(n + 2, VI(2, 0));
  REP(i, 1, n + 2) {
    REP(y, 0, 2) {
      int next = y ? c[i] : b[i];
      int mi = 1e8;
      REP(x, 0, 2) {
	int prev = x ? b[i - 1] : c[i - 1];
	mi = min(mi, dp[i - 1][x] + abs(next - prev) + abs(b[i] - c[i]));
      }
      dp[i][y] = mi;
    }
  }
  return min(dp[n + 1][0], dp[n + 1][1]);
}


int main(void) {
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  cout << solve(a) / 2 << endl;
}
