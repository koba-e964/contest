#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

#define LOCAL 0


int n;
#if LOCAL
const int N = 501;
int board[N][N];
void init(void) {
  REP(i, 0, n) {
    string s;
    cin >> s;
    REP(j, 0, n)
      board[i + 1][j + 1] = s[j] == '#' ? 1 : 0;
  }
}
bool ask(int r1, int c1, int r2, int c2) {
  assert (r1 <= r2);
  assert (c1 <= c2);
  assert (r2 - r1 + c2 - c1 >= n - 1);
  if (board[r1][c1] == 1 || board[r2][c2] == 1) return false;
  queue<PI> que;
  que.push(PI(r1, c1));
  int dx[2] = {0, 1};
  int dy[2] = {1, 0};
  vector<vector<bool> > vis(n + 1, vector<bool>(n + 1));
  while (not que.empty()) {
    PI p = que.front(); que.pop();
    int x = p.first;
    int y = p.second;
    if (board[x][y] == 1) continue;
    if (vis[x][y]) continue;
    vis[x][y] = true;
    REP(d, 0, 2) {
      int nx = x + dx[d];
      int ny = y + dy[d];
      if (nx <= n && ny <= n) {
	que.push(PI(nx, ny));
      }
    }
  }
  return vis[r2][c2];
}
#else
void init(void) {}
bool ask(int r1, int c1, int r2, int c2) {
  assert (r1 <= r2);
  assert (c1 <= c2);
  assert (r2 - r1 + c2 - c1 >= n - 1);
  cout << "? " << r1 << " " << c1 << " " << r2 << " " << c2 << endl;
  string res;
  cin >> res;
  return res == "YES";
}
#endif

int main(void) {
  cin >> n;
  init();
  vector<PI> pts;
  vector<PI> pts_rev;
  int x = 1, y = 1;
  // greedily to leftdown
  REP(i, 0, n - 1) {
    pts.push_back(PI(x, y));
    if (x < n && ask(x + 1, y, n, n)) {
      x++;
    } else {
      y++;
    }
  }
  pts.push_back(PI(x, y));
  x = n; y = n;
  REP(i, 0, n - 1) {
    pts_rev.push_back(PI(x, y));
    if (y > 1 && ask(1, 1, x, y - 1)) {
      y--;
    } else {
      x--;
    }
  }
  pts_rev.push_back(PI(x, y));
  assert (pts.back() == pts_rev.back());
  pts_rev.pop_back();
  reverse(pts_rev.begin(), pts_rev.end());
  if (0) {
    for (PI p: pts) cerr << " " << p.first << "," << p.second;
    cerr << endl;
    for (PI p: pts_rev) cerr << " " << p.first << "," << p.second;
    cerr << endl;
  }
  for (PI v: pts_rev) pts.push_back(v);
  assert ((int) pts.size() == 2 * n - 1);
  cout << "! ";
  REP(i, 0, pts.size() - 1) {
    if (pts[i].first == pts[i + 1].first) {
      cout << "R";
    } else {
      cout << "D";
    }
  }
  cout << endl;
}
