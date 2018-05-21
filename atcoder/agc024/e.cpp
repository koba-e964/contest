#include <iostream>
#include <cstring>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;

ll mod;

void add(ll &x, ll y) {
  x = (x + y) % mod;
}

const int N = 310;

ll dp[N][N], dp2[N][N];

// This solution is implemented after the author read the editorial.
int main(void) {
  int n, k;
  cin >> n >> k >> mod;
  dp[0][0] = 1;
  REP(i, 0, k) {
    memset(dp2, 0, sizeof(dp2));
    REP(j, 0, N) dp2[j][j] = dp[i][j];
    REP(j, 0, N) {
      for (int l = N - 1; l >= 0; --l) {
	if (j < N - 1) {
	  add(dp2[j + 1][l], dp2[j][l] * (l + 1));
	}
	if (l > 0) {
	  add(dp2[j][l - 1], dp2[j][l]);
	}
      }
    }
    REP(j, 0, N) {
      dp[i + 1][j] = dp2[j][0];
    }
  }
  cout << dp[k][n] << endl;
}
