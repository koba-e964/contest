#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef vector<int> VI;

int main(void){
  int n;
  cin >> n;
  vector<bool> tbl(n + 1, true);
  tbl[0] = tbl[1] = false;
  REP(i, 2, 200) {
    REP(j, 2, n / i + 1) {
      tbl[i * j] = false;
    }
  }
  VI primes;
  REP(i, 0, n + 1) { if(tbl[i]) primes.push_back(i); }
  int m = primes.size();

  vector<VI> dp(m + 1, VI(n + 1, -1));
  dp[0][0] = 0;
  REP(i, 0, m) {
    REP(j, 0, n + 1) {
      int r = dp[i][j];
      if (j >= primes[i] && dp[i][j - primes[i]] >= 0) {
	r = max(r, dp[i][j - primes[i]] + 1);
      }
      dp[i + 1][j] = r;
    }
  }
  cout << dp[m][n] << endl;
}
