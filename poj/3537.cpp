#include <cstdio>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)


using namespace std;
typedef vector<int> VI;


int main(void){
  int n;
  scanf("%d", &n);
  VI dp(n + 1);
  dp[0] = 0;
  dp[1] = 1;
  REP(i, 2, n + 1) {
    VI tbl(i + 1, 0);
    REP(j, 0, i) {
      tbl[dp[max(j - 2, 0)] ^ dp[max(i - j - 3, 0)]] = 1;
    }
    // Find the mex
    int cur = 0;
    while (tbl[cur] >= 1) {
      cur += 1;
    }
    dp[i] = cur;
  }
  printf("%d\n", dp[n] == 0 ? 2 : 1);
}
