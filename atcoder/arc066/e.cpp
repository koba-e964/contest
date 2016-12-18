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
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 100100;

ll minf = -1e16;
ll dp[N][3];

int main(void){
  int n;
  cin >> n;
  VL a(n);
  vector<char> ops(n - 1);
  REP(i, 0, n - 1) {
    cin >> a[i] >> ops[i];
  }
  cin >> a[n - 1];
  REP(i, 0, n + 1) {
    REP(j, 0, 3) {
      dp[i][j] = minf;
    }
  }
  dp[0][0] = a[0];
  REP(i, 1, n) {
    REP(j, 0, 3) {
      if (ops[i - 1] == '+') {
	dp[i][j] = dp[i - 1][j] + (j == 1 ? -a[i] : a[i]);
      } else {
	dp[i][j] = max(dp[i][j], dp[i - 1][j] + (j == 0 ? -a[i] : a[i]));
	if (j < 2) {
	  dp[i][j + 1] = max(dp[i][j + 1], dp[i - 1][j] + (j == 0 ? -a[i] : a[i]));
	}
      }
    }
  }
  ll ma = minf;
  REP(i, 0, 3) {
    ma = max(ma, dp[n - 1][i]);
  }
  cout << ma << endl;
}
