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

vector<PI> rows, cols;
const int inf = 1e6;

int dp[101][101];
int dp2[101][101];
int solve(int s, const vector<PI> conf) {
  if (s >= 101) {
    assert(!"s >= 101");
  }
  REP(i, 0, 101) {
    REP(j, 0, s) {
      dp[i][j] = 0;
    }
  }
  REP(i, 0, conf.size()) {
    PI t = conf[i];
    dp[t.first][t.second] = 1;
  }
  REP(x, 0, s) {
    dp2[0][x] = dp[0][x] ? inf : 0;
  }
  REP(i, 1, 101) {
    REP(x, 0, s) {
      if (dp[i][x]) {
	dp2[i][x] = inf;
	continue;
      }
      int mi = inf;
      REP(d, 0, s) {
	mi = min(mi, dp2[i - 1][d] + abs(x - d));
      }
      dp2[i][x] = mi;
    }
  }
  int mi = inf;
  REP(x, 0, s) {
    mi = min(dp2[100][x], mi);
  }
  return mi;
}


int main(void){
  int w, h, q;
  cin >> w >> h >> q;
  REP(i, 0, q) {
    int t, d, x;
    cin >> t >> d >> x;
    t--;
    x--;
    if (d == 0) {
      cols.push_back(PI(t, x));
    } else {
      rows.push_back(PI(t, x));
    }
  }
  int res = solve(h, rows) + solve(w, cols);
  cout << (res >= inf ? -1 : res) << endl; 
}
