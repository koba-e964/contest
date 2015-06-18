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

const int N = 101;

double dp[N][N][N];
int n;

void solve(void) {
  dp[0][0][0] = 0.0;
  REP(k, 0, n + 1) {
    REP(j, 0, n + 1) {
      REP(i, 0, n + 1) {
	if (i + j + k == 0) {
	  continue;
	}
	if (i + j + k > n) {
	  continue;
	}
	double d = i + j + k;
	double sum = (double)n / d;
	if (i >= 1) {
	  sum += dp[i - 1][j][k] * i / d;
	}
	if (j >= 1) {
	  sum += dp[i + 1][j - 1][k] * j / d;
	}
	if (k >= 1) {
	  sum += dp[i][j + 1][k - 1] * k / d;
	}
	dp[i][j][k] = sum;
      }
    }
  }
}

int main(void){
  cin >> n;
  int a0 = 0, a1 = 0, a2 = 0;
  REP(i, 0, n) {
    int t;
    cin >> t;
    switch(t) {
    case 0:
      a0++; break;
    case 1:
      a1++; break;
    case 2:
      a2++; break;
    default:
      ;
    }
  }
  solve();
  printf("%.9f\n", dp[a2][a1][a0]);
}
