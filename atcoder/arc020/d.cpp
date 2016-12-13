#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
const ll mod = 1e9 + 7;

// Solution in the editorial
ll solve(int m, int k, VI &d) {
  int n = d.size() + 1;
  vector<vector<VL> > dp(n, vector<VL>(1 << k, VL(m)));
  dp[0][0][0] = 1;
  REP(i, 1, n) {
    REP(bits, 0, 1 << k) {
      int tcnt = 0;
      REP(j, 0, k - 1) {
	if (((bits >> j) & 1) != ((bits >> (j + 1)) & 1)) {
	  tcnt++;
	}
      }
      REP(j, 0, k) {
	if ((bits & 1 << j) == 0) { continue; }
	REP(l, 0, m) {
	  ll &ret = dp[i][bits][(l + d[i - 1] * tcnt) % m];
	  ret += dp[i - 1][bits ^ 1 << j][l];
	  ret %= mod;
	}
      }
      REP(l, 0, m) {
	ll &ret = dp[i][bits][(l + d[i - 1] * tcnt) % m];
        ret += dp[i - 1][bits][l];
	ret %= mod;
      }
    }
  }
  ll tot = dp[n - 1][(1 << k) - 1][0] % mod;
  REP(i, 0, k) {
    tot += dp[n - 1][((1 << k) - 1) ^ 1 << i][0];
    tot %= mod;
  }
  return tot;
}

int main(void){
  int n, m, k;
  cin >> n >> m >> k;
  VI d(n - 1);
  REP(i, 0, n - 1) {
    cin >> d[i];
  }
  vector<VI> pd(n, VI(n));
  REP(i, 0, n) {
    pd[i][i] = 0;
    REP(j, 0, i) {
      pd[i][i - j - 1] = pd[i][i - j] + d[i - j - 1];
      pd[i][i - j - 1] %= m;
    }
    REP(j, i + 1, n) {
      pd[i][j] = pd[i][j - 1] + d[j - 1];
      pd[i][j] %= m;
    }
  }
  cout << solve(m, k, d) << endl;
}
