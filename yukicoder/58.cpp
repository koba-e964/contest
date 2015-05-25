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

int n,k;

const int N = 100;
double dp1[N][N] = {0}, dp2[N][N] = {0};

void calc(double dp[][N], int q) {
  dp[0][0] = 1;
  REP (i, 1, q + 1) {
    REP (j, 0, 6 * n + 1) {
      REP (x, 1, 7) {
	if (j >= x) {
	  dp[i][j] += dp[i-1][j - x] / 6.0;
	}
      }
    }
  }
  REP (i, q + 1, n + 1) {
    REP (j, 0, 6 * n + 1) {
      REP (x, 4, 7) {
	if (j >= x) {
	  dp[i][j] += dp[i-1][j - x] / 3.0;
	}
      }
    }
  }
}

int main(void){
  cin >> n >> k;
  calc(dp1, n - k);
  calc(dp2, n);
  double sum = 0;
  REP(i, 0, n * 6 + 1) {
    REP (j , 0, i) {
      sum += dp1[n][i] * dp2[n][j];
    }
  }
  printf("%.9f\n", sum);
}
