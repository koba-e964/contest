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
const int inf = 1e8;

const int N = 801;
string a[N];

int tbl[N][N];

int main(void){
  int h, w, k;
  cin >> h >> w >> k;
  REP(i, 0, h) {
    cin >> a[i];
  }
  // first k steps
  typedef pair<PI, int> PPII;
  int sx = -1, sy = -1;
  REP(i, 0, h) {
    REP(j, 0, w) {
      if (a[i][j] == 'S') {
	sx = i;
	sy = j;
      }
    }
  }
  REP(i, 0, h) {
    REP(j, 0, w) {
      tbl[i][j] = inf;
    }
  }
  queue<PPII> que;
  que.push(PPII(PI(sx, sy), 0));
  int dx[4] = {1, 0, -1, 0};
  int dy[4] = {0, 1, 0, -1};
  while (not que.empty()) {
    PPII t = que.front(); que.pop();
    int x = t.first.first;
    int y = t.first.second;
    int turn = t.second;
    if (tbl[x][y] <= turn) {
      continue;
    }
    tbl[x][y] = turn;
    if (turn >= k) { continue; }
    REP(d, 0, 4) {
      int nx = x + dx[d];
      int ny = y + dy[d];
      if (nx < 0 || nx >= h || ny < 0 || ny >= w) {
	continue;
      }
      if (a[nx][ny] == '#') { continue; }
      que.push(PPII(PI(nx, ny), turn + 1));
    }
  }
  int mi = inf;
  REP(i, 0, h) {
    mi = min(mi, tbl[i][0]);
    mi = min(mi, tbl[i][w - 1]);
  }
  REP(i, 0, w) {
    mi = min(mi, tbl[0][i]);
    mi = min(mi, tbl[h - 1][i]);
  }
  if (mi != inf) {
    cout << (mi + k - 1) / k << endl;
    return 0;
  }
  // bfs without blocks
  REP(i, 0, h) {
    REP(j, 0, w) {
      if (tbl[i][j] != inf) {
	que.push(PPII(PI(i, j), k));
      }
      tbl[i][j] = inf;
    }
  }
  while (not que.empty()) {
    PPII t = que.front(); que.pop();
    int x = t.first.first;
    int y = t.first.second;
    int turn = t.second;
    if (tbl[x][y] <= turn) {
      continue;
    }
    tbl[x][y] = turn;
    REP(d, 0, 4) {
      int nx = x + dx[d];
      int ny = y + dy[d];
      if (nx < 0 || nx >= h || ny < 0 || ny >= w) {
	continue;
      }
      que.push(PPII(PI(nx, ny), turn + 1));
    }
  }
  if (0) {
    REP(i, 0, h) {
      cerr << "tbl[" << i << "]:";
      REP(j, 0, w) {
	cerr << " " << tbl[i][j];
      }
      cerr << endl;
    }
  }
  mi = inf;
  REP(i, 0, h) {
    mi = min(mi, tbl[i][0]);
    mi = min(mi, tbl[i][w - 1]);
  }
  REP(i, 0, w) {
    mi = min(mi, tbl[0][i]);
    mi = min(mi, tbl[h - 1][i]);
  }
  assert (mi != inf);
  cout << (mi + k - 1) / k << endl;
}
