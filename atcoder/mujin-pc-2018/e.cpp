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

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, PI> PLPI;

const int inf = 1e8;
const ll linf = 1e17;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m, k;
  string dtape;
  cin >> n >> m >> k >> dtape;
  vector<string> s(n);
  int sx = -1, sy = -1, gx = -1, gy = -1;
  REP(i, 0, n) {
    cin >> s[i];
    REP(j, 0, m) {
      if (s[i][j] == 'S') {
	sx = i, sy = j;
      }
      if (s[i][j] == 'G') {
	gx = i, gy = j;
      }
    }
  }
  int dx[4] = {1, 0, -1, 0};
  int dy[4] = {0, 1, 0, -1};
  string dir = "DRUL";
  vector<VI> nxt(4, VI(k, inf));
  for (int i = 2 * k - 1; i >= 0; --i) {
    int r = i % k;
    int nr = (i + 1) % k;
    REP(d, 0, 4) {
      if (dtape[r] == dir[d]) {
	nxt[d][r] = 0;
      }
    }
    REP(d, 0, 4) {
      nxt[d][r] = min(nxt[d][r], nxt[d][nr] + 1);
    }
  }
  if (DEBUG) {
    REP(i, 0, k) {
      REP(d, 0, 4) {
	cerr << " " << nxt[d][i];
      }
      cerr << endl;
    }
  }
  priority_queue<PLPI, vector<PLPI>, greater<PLPI> > que;
  que.push(PLPI(0, PI(sx, sy)));
  vector<VL> dp(n, VL(m, linf));
  while (not que.empty()) {
    PLPI top = que.top(); que.pop();
    ll dist = top.first;
    int vx = top.second.first, vy = top.second.second;
    int r = dist % k;
    if (s[vx][vy] == '#') continue;
    if (dp[vx][vy] <= dist) continue;
    dp[vx][vy] = dist;
    REP(d, 0, 4) {
      int nx = vx + dx[d];
      int ny = vy + dy[d];
      if (nx < 0 || nx >= n || ny < 0 || ny >= m) continue;
      if (nxt[d][r] >= inf) continue;
      que.push(PLPI(dist + nxt[d][r] + 1, PI(nx, ny)));
    }
  }
  if (DEBUG) {
    REP(i, 0, n) {
      REP(j, 0, m) {
	cerr << " " << dp[i][j];
      }
      cerr << endl;
    }
  }
  cout << (dp[gx][gy] >= linf ? -1 : dp[gx][gy]) << endl;
}
