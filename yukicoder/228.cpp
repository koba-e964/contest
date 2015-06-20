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

int a[4][4];

int c[4][4] = {{1,2,3,4},{5,6,7,8},{9,10,11,12},{13,14,15, 0}};

int dp[4][4];

int main(void){
  int sx = -1, sy;
  REP(i, 0, 4) {
    REP(j , 0, 4) {
      cin >> a[i][j];
      dp[i][j] = a[i][j] != c[i][j];
      if (a[i][j] == 0) {
	sx = i;
	sy = j;
      }
    }
  }
  queue<PI> que;
  que.push(PI(sx, sy));
  int dx[4] = {1,0,-1,0}, dy[4] = {0, 1, 0, -1};
  while (!que.empty()) {
    PI p = que.front(); que.pop();
    REP(i, 0, 4  ) {
      int nx = p.first + dx[i];
      int ny = p.second + dy[i];
      dp[p.first][p.second] = 0;
      if (nx < 0 || nx >= 4 || ny < 0 || ny >= 4 || dp[nx][ny] == 0) {
	continue;
      }
      if (a[nx][ny] == c[p.first][p.second]) {
        que.push(PI(nx, ny));
        break;
      }
    }
  }
  bool ok = 1;
  REP(i, 0, 4) REP(j, 0, 4) {
    ok &= ! dp[i][j];
  }
  cout << (ok ? "Yes" : "No") << endl;
}
