#include <cstdio>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 1010;
double dp[N][N][2];

int main(void){
  int n, k;
  cin >> n >> k;
  REP(j, 0, k + 1) {
    dp[n][j][1] = 1;
  }
  for (int i = n - 1; i >= 0; --i) {
    REP(j, 0, min(k, i) + 1) {
      REP(b, 0, 2) {
	dp[i][j][b] = (max(dp[i + 1][j + 1][1], dp[i + 1][j][0])
		       + dp[i + 1][j][b] * i) / (i + 1);
      }
    }
  }
  printf("%.10f\n", dp[0][0][0]);
}
