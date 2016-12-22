#include <cstdio>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

const int W = 150100;

int dp[2][W];
const int inf = 1e9;

int main(void){
  int n, p;
  cin >> n >> p;
  REP(j, 0, W) {
    dp[0][j] = dp[1][j] = inf;
  }
  dp[0][0] = 0;
  REP(i, 1, n + 1) {
    int t = i % 2;
    int a[4];
    REP(j, 0, 3) { cin >> a[j]; }
    a[3] = 1;
    REP(j, 0, 3 * i + 1) {
      int res = inf;
      REP(k, 0, 4) {
	if (j >= k) {
	  res = min(res, dp[1 - t][j - k] + a[k]);
	}
      }
      dp[t][j] = res;
    }
  }
  printf("%.15f\n", dp[n % 2][p] / 1.0 / n);
}
