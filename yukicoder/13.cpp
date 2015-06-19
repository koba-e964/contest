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

int w, h;
const int W = 101;
int m[W][W];

int dp[W][W] = {};

int col(int x, int y) {
  if (x < 0 || x >= h || y < 0 || y >= w) {
    return -1;
  }
  return m[x][y];
}

int dfs(int x, int y, int bx, int by) {
  if (bx == -1 && by == -1 && dp[x][y]) {
    return 0;
  }
  if (dp[x][y]) {
    return 1;
  }
  dp[x][y] = 1;
  int dx[4] = {1, 0, -1, 0}, dy[4] = {0, 1, 0, -1};
  REP(i, 0, 4) {
    int nx = x + dx[i];
    int ny = y + dy[i];
    if (col(nx, ny) != m[x][y]) {
      continue;
    }
    if (nx != bx || ny != by) {
      int sub = dfs(nx, ny, x, y);
      if (sub) {
	return sub;
      }
    }
  }
  return 0;
}

int main(void){
  cin >> w >> h;
  REP(i, 0, h) {
    REP(j, 0, w) {
      cin >> m[i][j];
    }
  }
  int res = 0;
  REP(i, 0, h) {
    REP(j, 0, w) {
      res |= dfs(i, j, -1, -1);
    }
  }
  cout << (res ? "possible" : "impossible") << endl;
}
