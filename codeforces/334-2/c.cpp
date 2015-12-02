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

const int N = 1e5 + 10;

int dp[2][3][N];


int main(void){
  int n;
  cin >> n;
  string s;
  cin >> s;
  REP(j, 0, 2) {
    REP(k, 0, 3) {
      dp[j][k][0] = 0;
    }
  }
  REP(i, 0, n) {
    int d = s[i] - '0';
    REP(k, 0, 3) {
      dp[d][k][i + 1] = dp[1 - d][k][i] + 1;
      dp[1 - d][k][i + 1] = dp[1 - d][k][i];
    }
    REP(k, 0, 2) {
      dp[d][k + 1][i + 1] = max(dp[d][k + 1][i + 1], dp[d][k][i] + 1);
    }
  }
  int m = 0;
  REP(j, 0, 2) {
    REP(k, 0, 3) {
      m = max(dp[j][k][n], m);
    }
  }
  cout << m << endl;
}
