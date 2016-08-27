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

const int N = 110;
int dp[N][N];
const int inf = 1e8;

int main(void){
  string s;
  cin >> s;
  int n = s.length();
  REP(i, 0, n + 1) {
    REP(j, 0, n + 1) {
      dp[i][j] = inf;
    }
  }
  dp[0][0] = 0;
  REP(i, 1, n + 1) {
    REP(j, 0, n + 1) {
      if (j >= 1) {
	// '<'
	dp[i][j] = min(dp[i][j], dp[i - 1][j - 1] + (s[i - 1] == '(' ? 0 : 1));
      }
      if (j < n) {
	// ')'
	dp[i][j] = min(dp[i][j], dp[i - 1][j + 1] + (s[i - 1] == ')' ? 0 : 1));
      }
    }
  }
  int mi = inf;
  int cur = 0;
  for (int i = n; i >= 0; --i) {
    if (cur < 0) {
      break;
    }
    if (cur >= 0) {
      mi = min(mi, dp[i][cur] + max(0, i - 1));
    }
    if (i >= 1) {
      cur += s[i - 1] == ')' ? 1 : -1;
    }
  }
  cout << mi << endl;
}
