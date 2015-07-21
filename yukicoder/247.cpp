#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;


const int N = 101, W = 100010;
const int inf = 2e7;

int dp[N][W];
int a[N];

int main(void){
  int c, n;
  cin >> c >> n;
  REP(i, 0, n) {
    cin >> a[i];
  }
  dp[0][0] = 0;
  REP(j, 1, W) {
    dp[0][j] = inf;
  }
  REP(i, 1, n + 1) {
    REP(j, 0, W) {
      if (j < a[i - 1]) {
	dp[i][j] = dp[i - 1][j];
      } else {
	dp[i][j] = min(dp[i - 1][j], dp[i][j - a[i - 1]] + 1);
      }
    }
  }
  int res = dp[n][c];
  cout << (res == inf ? -1 : res) << endl;
}
