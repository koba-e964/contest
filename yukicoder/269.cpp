#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;


const ll mod = 1e9 + 7;

const int N = 101;
const int S = 20010;
ll dp[N][S];



int main(void){
  int n,s,k;
  cin >> n >> s >> k;
  int r = s - k * n * (n - 1) / 2;
  if (r < 0) {
    cout << 0 << endl;
    return 0;
  }
  REP(i, 0, S) {
    dp[0][i] = 0;
  }
  dp[0][0] = 1;

  REP(i, 0, n) {
    REP(j, 0, S) {
      int w = i + 1;
      ll sub = dp[i][j];
      if (j >= w) {
	sub += dp[i + 1][j - w];
      }
      sub %= mod;
      dp[i + 1][j] = sub;
    }
  }
  cout << dp[n][r] << endl;
}
