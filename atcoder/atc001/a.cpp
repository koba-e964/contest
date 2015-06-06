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

const int H = 501;
string s[H];
int h, w;
int sx,sy;
int dp[H][H] = {0};
int dfs(int x, int y) {
  if (x < 0 || x >= h || y <0 || y >= w) {
    return 0;
  }
  if (s[x][y] == '#') {
    return 0;
  }
  if (s[x][y] == 'g') {
    return 1;
  }
  if (dp[x][y] == 2) {
    return 0;
  }
  dp[x][y] = 2;
  int dx[4] = { 1,0,-1,0}, dy[4] = {0,1,0,-1};
  REP(i, 0, 4) {
    int nx = x + dx[i];
    int ny = y + dy[i];
    int r = dfs(nx, ny);
    if (r) return 1;
  }
  return 0;
}
int main(void){
  cin >> h >> w;
  REP(i, 0, h) {
    cin >> s[i];
    int k = find(s[i].begin(), s[i].end(), 's') - s[i].begin();
    if (k < w) {
      sx = i, sy = k;
    }
  }
  cout << (dfs(sx, sy) ? "Yes" : "No") << endl;
}
