#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;


const int W = 12345;
int dp[2][114][W];
const int N = 123;
int a[N], b[N], c[N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, cc;
  cin >> n >> cc;
  REP(i, 0, n) {
    cin >> a[i] >> b[i] >> c[i];
  }
  REP(i, 0, cc + 1) {
    REP(j, 0, W) {
      dp[0][i][j] = -1;
    }
  }
  dp[0][0][0] = 0;
  REP(i, 0, n) {
    int t = i % 2;
    REP(i, 0, cc + 1) {
      REP(j, 0, W) {
	dp[1 - t][i][j] = -1;
      }
    }
    REP(j, c[i], cc + 1) {
      REP(k, 0, W) {
	int &ret = dp[1 - t][j][k];
	if (k >= a[i]) {
	  if (dp[t][j - c[i]][k - a[i]] >= 0) {
	    ret = max(ret, dp[t][j - c[i]][k - a[i]] + b[i]);
	  }
	}
	if (k >= b[i]) {
	  if (dp[t][j - c[i]][k - b[i]] >= 0) {
	    ret = max(ret, dp[t][j - c[i]][k - b[i]] + a[i]);
	  }
	}
      }
    }
    REP(j, 0, cc + 1) {
      REP(k, 0, W) {
	dp[1 - t][j][k] = max(dp[1 - t][j][k], dp[t][j][k]);
      }
    }
  }
  int ma = 0;
  REP(i, 0, cc + 1) {
    REP(j, 0, W) {
      ma = max(ma, min(dp[n % 2][i][j], j));
    }
  }
  cout << ma << "\n";
}
