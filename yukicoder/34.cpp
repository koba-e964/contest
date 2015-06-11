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
const int N = 100;
const int V = 2000;
int n, v, sx,sy ,gx, gy;
int l[N][N];
int dp[N][N][V];
const int inf = 10000000;

int main(void){
  cin >> n >> v >> sx >> sy >> gx >> gy;
  sx--, sy--, gx--, gy--;
  REP(i, 0, n) {
    REP(j, 0, n) {
      cin >> l[i][j];
    }
  }
  if (v >= V) {
    cout << abs(sx - gx) + abs(sy - gy) << endl;
    return 0;
  }
  REP(i, 0, n) {
    REP(j, 0, n) {
      fill_n(dp[i][j], V, inf);
    }
  }
  typedef pair<int, PI> qtt;
  queue<qtt> que;
  que.push(qtt(0, PI(sy * n + sx, v)));
  while (!que.empty()) {
    qtt p = que.front(); que.pop();
    int x = p.second.first;
    int y = x % n;
    int hp = p.second.second;
    x /= n;
    if (dp[x][y][hp] <= p.first) {
      continue;
    }
    dp[x][y][hp] = p.first;
    int dx[4] = {1,0,-1,0}, dy[4] = {0,1,0,-1};
    REP(d, 0, 4) {
      int nx = x + dx[d];
      int ny = y + dy[d];
      if (nx < 0 || nx >= n || ny < 0 || ny >= n) {
	continue;
      }
      int newhp = hp - l[nx][ny];
      if (newhp <= 0 || dp[nx][ny][newhp] < inf) {
	continue;
      }
      que.push(qtt(p.first + 1, PI(nx * n + ny, newhp)));
    }
  }
  int mi = inf;
  REP(i, 0, V) {
    mi = min(mi, dp[gy][gx][i]);
  }
  cout << (mi == inf ? -1 : mi) << endl;
}
