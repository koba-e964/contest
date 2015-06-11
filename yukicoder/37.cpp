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

const int  N = 140;
const int V = 501 * N;
int b[N];
int v[N];

int cc[16];
int vv[16];
int dp[N][V];

int main(void){
  int t, n;
  cin >> t >> n;
  REP(i, 0, n) {
    cin >> cc[i];
  }
  REP(i, 0, n) {
    cin >> vv[i];
  }
  int c = 0;
  REP(i, 0, n) {
    int q = vv[i];
    while (q > 0) {
      b[c] = cc[i];
      v[c] = q;
      q /= 2;
      ++c;
    }
  }
  REP(j, 0, V) {
    dp[0][j] = 0;
  }
  REP(i, 1, c + 1) {
    REP(j, 0, V) {
      int ma = 0;
      ma = dp[i - 1][j];
      if (j >= b[i - 1]) {
	ma = max(ma, dp[i - 1][j - b[i - 1]] + v[i - 1]);
      }
      if (j >= 1) {
	ma = max(ma, dp[i][j - 1]);
      }
      dp[i][j] = ma;
    }
  }
  cout << dp[c][t] << endl;
}
