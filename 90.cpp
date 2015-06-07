#include <algorithm>
#include <iostream>
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

const int N = 9;
int dp[1 << N];
int mat[N][N];

int main(void){
  int n, m;
  cin >> n >> m;
  REP (i, 0, m) {
    int i1, i2, s;
    cin >> i1 >> i2 >> s;
    mat[i1][i2] = s;
  }
  REP(bits, 0, 1 << n) {
    int ma = 0;
    REP(i, 0, n) {
      if ((bits & (1 << i)) == 0) {
	continue;
      }
      int gain = 0;
      REP (j, 0, n) {
	if ((bits & (1 << j)) == 0) {
	  continue;
	}
	gain += mat[j][i];
      }
      ma = max(dp[bits ^ (1 << i)] + gain, ma);
    }
    dp[bits] = ma;
  }
  cout << dp[(1 << n) - 1] << endl;
}
