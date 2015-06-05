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
const int N = 30;
string s[30];

int first[N][N] = {0};
int dp[N][N] = {0};
int dx[4] = {1,0,-1,0}, dy[4] = {0, 1, 0, -1};

int h, w;
int read(int x, int y) {
  if (x < 0 || x >= h || y < 0 || y >= w) {
    return 0;
  }
  return s[x][y] == '.';
}

void ff(int x, int y) {
  if (read(x, y) == 0) {
    return;
  }
  first[x][y] = 1;
  REP (i, 0, 4) {
    int nx = x + dx[i];
    int ny = y + dy[i];
    if (read(nx, ny) && first[nx][ny] == 0) {
      ff(nx, ny);
    }
  }
}

int main(void){
  cin >> w >> h;
  int x = -1,y;
  REP(i, 0, h) {
    cin >> s[i];
    REP(j, 0, w) {
      if (x == -1 && s[i][j] == '.') {
	x = i, y = j;
      }
    }
    fill_n(dp[i], w, -1);
  }
  ff(x, y);
  queue<pair<PI, int> > que;
  REP(i,0, h) {
    REP(j, 0, w) {
      if (first[i][j]) {
	que.push(pair<PI,int>(PI(i, j), 0));
      }
    }
  }
  int mi = 1000000;
  while (! que.empty()) {
    pair<PI, int> pi = que.front(); que.pop();
    PI p = pi.first;
    if (dp[p.first][p.second] >= 0) {
      continue;
    }
    dp[p.first][p.second] = pi.second;
    if (read(p.first, p.second)&& pi.second) {
      mi = min(mi, pi.second);
    }
    REP(i, 0, 4) {
      int nx = p.first + dx[i], ny = p.second + dy[i];
      int nv = pi.second + 1;
      que.push(pair<PI, int>(PI(nx,ny), nv));
    }
  }
  cout << mi - 1 << endl;
}
