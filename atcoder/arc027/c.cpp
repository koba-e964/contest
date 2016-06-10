#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
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

const int N = 301;
const int B = 700;

int dp[N][N][B];

int main(void){
  int x, y, n;
  cin >> x >> y >> n;
  VI t(n);
  vector<int> h(n);
  REP(i, 0, n) {
    cin >> t[i] >> h[i];
  }
  REP(i, 0, n) {
    REP(j, 0, i + 1) {
      REP(k, 0, B) {
	dp[i + 1][j][k] = max(dp[i + 1][j][k], dp[i][j][k]);
	if (k < B - t[i]) {
	  dp[i + 1][j + 1][k + t[i]] = max(dp[i + 1][j + 1][k + t[i]], dp[i][j][k] + h[i]);
			   
	}
      }
    }
  }
  int ma = 0;
  int nx = min(x, n);
  REP(j, 0, nx + 1) {
    REP(k, 0, x + y + 1) {
      ma = max(ma, dp[n][j][k]);
    }
  }
  cout << ma << endl;
}
