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

const int W = 10010;
const int N = 51;
int dp[N][N][W];


int main(void){
  int w, n, k;
  cin >> w >> n >> k;
  REP(i, 0, n) {
    int a, b;
    cin >> a >> b;
    REP(j, 0, i + 1) {
      REP(l, 0, W) {
	dp[i + 1][j][l] = max(dp[i + 1][j][l], dp[i][j][l]);
	if (l < W - a) {
	  dp[i + 1][j + 1][l + a] = max(dp[i + 1][j + 1][l + a], dp[i][j][l] + b);
	}
      }
    }
  }
  int ma = 0;
  REP(j, 0, k + 1) {
    REP(l, 0, w + 1) {
      ma = max(ma, dp[n][j][l]);
    }
  }
  cout << ma << endl;
  
}
