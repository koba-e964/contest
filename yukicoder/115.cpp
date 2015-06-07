#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;

int dp[101][1010][11];

int main(void){
  int n, d, k;
  cin >> n >> d >> k;
  dp[n][0][0] = 1;
  for (int i = n - 1; i >= 0; --i) {
    REP(j, 0, d + 1) {
      REP(l, 0, k + 1) {
	dp[i][j][l] = dp[i + 1][j][l];
	if (j >= i + 1 && l >= 1) {
	  dp[i][j][l] |= dp[i + 1][j - i - 1][l - 1];
	}
      }
    }
  }
  if (dp[0][d][k] == 0) {
    cout << -1 << endl;
    return 0;
  }
  int cur = d, curk = k;
  REP (i, 0, n) {
    if (curk > 0 && dp[i + 1][d - i - 1][curk - 1]) {
      cout << i + 1;
      if (curk > 1) {
	cout << " ";
      }
      d -= i + 1;
      curk--;
    }
  }
  cout << endl;
}
