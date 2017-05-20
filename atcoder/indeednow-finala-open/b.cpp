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
typedef pair<PI, int> PPII;

string a[200];

int main(void){
  int r, c;
  cin >> r >> c;
  PI s, t;
  REP(i, 0, r) {
    cin >> a[i];
    REP(j, 0, c) {
      if (a[i][j] == 's') {
	s = PI(i, j);
      } else if (a[i][j] == 't') {
        t = PI(i, j);
      }
    }
  }
  queue<PPII> que;
  que.push(PPII(s, 0));
  vector<VI> dist(r, VI(c, 1e8));
  while (not que.empty()) {
    PPII t = que.front(); que.pop();
    int dis = t.second;
    int x = t.first.first;
    int y = t.first.second;
    if (dist[x][y] <= dis) { continue; }
    dist[x][y] = dis;
    int dx[6] = {1, 1, 0, -1, -1, 0};
    int dy[6] = {0, 1, 1, 0, 1, -1};
    if (x % 2 == 0) {
      dy[1] = -1;
      dy[4] = -1;
    }
    REP(d, 0, 6) {
      int nx = x + dx[d];
      int ny = y + dy[d];
      if (nx < 0 || nx >= r || ny < 0 || ny >= c) {
	continue;
      }
      int weight = isdigit(a[nx][ny]) ? a[nx][ny] - '0' : 0;
      que.push(PPII(PI(nx, ny), dis + weight));
    }
  }
  cout << dist[t.first][t.second] << endl;
  
}
