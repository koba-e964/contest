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

int h, w;
const int N = 101;
int dp[N][N];

string m[N];

int canMove(int x, int y) {
  if (x >= h) {
    return 0;
  }
  if (y >= w) {
    return 0;
  }
  return m[x][y] == '.';
}

int solve(int x, int y) {
  if (dp[x][y] >= 0) {
    return dp[x][y];
  }
  int &ret = dp[x][y];
  int dx[3] = {1, 0, 1}, dy[3] = {0, 1, 1};
  REP(i, 0, 3) {
    if (canMove(x + dx[i], y + dy[i]) && solve(x + dx[i], y + dy[i]) == 0) {
      return ret = 1;
    }
  }
  return ret = 0;
}

int main(void){
  cin >> h >> w;
  REP(i, 0, h) {
    cin >> m[i];
  }
  REP(i, 0, h + 1) {
    REP(j ,0, w + 1) {
      dp[i][j] = -1;
    }
  }
  cout << (solve(0, 0) ? "First" : "Second") << endl;
}
