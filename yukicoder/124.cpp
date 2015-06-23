#include <iostream>
#include <algorithm>
#include <queue>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef pair<int, int> PI;

const int K = 11;
const int N = 101;
const int inf = 100000;
int m[N][N];
int dp[K][N][N];
struct dat {
  int p0, p1, cur;
  PI pos;
  int dist;
};

bool is_kado(int x, int y, int z) {
  if (x != y && y != z && z != x) {
    return !((x < y && y < z) || (x > y && y > z));
  }
  return 0;
}
int main(void){
  int w, h;
  cin >> w >> h;
  REP(i, 0, h) {
    REP(j, 0, w) {
      cin >> m[i][j];
      REP(k, 0, K) {
	dp[k][i][j] = inf;
      }
    }
  }
  queue<dat> que;
  que.push((dat) {-1, -1, m[0][0], PI(0,0), 0});
  while (! que.empty()) {
    dat p = que.front(); que.pop();
    if (p.p0 != -1 && p.p1 != -1 && ! is_kado(p.p0, p.p1, p.cur)) {
      continue;
    }
    int x = p.pos.first;
    int y = p.pos.second;
    if (p.p1 >= 0 && p.dist >= dp[p.p1][x][y]) {
      continue;
    }
    if (p.p1 >= 0) {
      dp[p.p1][x][y] = p.dist;
    }
    int dx[4] = {1,0,-1,0}, dy[4] = {0,1,0,-1};
    REP(i, 0, 4) {
      int nx = x + dx[i];
      int ny = y + dy[i];
      if (nx < 0 || nx >= h || ny < 0 || ny >= w) {
	continue;
      }
      que.push((dat) {p.p1, p.cur, m[nx][ny], PI(nx, ny), p.dist + 1});
    }
  }
  int mi = inf;
  REP(i, 0, K) {
    mi = min(mi, dp[i][h - 1][w - 1]);
  }
  cout << (mi == inf ? -1 : mi) << endl;
}
