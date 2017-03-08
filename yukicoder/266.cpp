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

const int W = 20;
const int inf = 10000;
int dp[W][W];

int main(void){
  int n;
  cin >> n;
  vector<int> s(n + 1);
  REP(i, 0, n + 1) {
    cin >> s[i];
  }
  REP(i, 0, W) {
    REP(j, 0, W) {
      dp[i][j] = inf;
    }
  }
  dp[0][0] = 1;
  REP(i, 0, n + 1) {
    REP(j, 0, s[i] + 1) {
      if (i == 0 && j == 0) { continue; }
      int mi = inf;
      REP(k, 0, j) {
	mi = min(mi, dp[i][j - k - 1] + dp[i][k]);
      }
      if (i >= 1 && j <= s[i - 1]) {
	mi = min(mi, dp[i - 1][j] + 1);
      }
      dp[i][j] = mi;
    }
  }
  REP(j, 0, s[n] + 1) {
    cout << dp[n][j] << (j == s[n] ? "\n" : " ");
  }
}
