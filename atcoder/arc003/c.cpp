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
const ll mod = 1e9 + 7;

const int N = 510;
string s[N];

int main(void){
  int n, m;
  cin >> n >> m;
  REP(i, 0, n) {
    cin >> s[i];
  }
  int sx = -1, sy = -1, gx = -1, gy = -1;
  REP(i, 0, n) {
    REP(j, 0, m) {
      if (s[i][j] == 's') {
	sx = i; sy = j;
      }
      if (s[i][j] == 'g') {
	gx = i, gy = j;
      }
    }
  }
  assert (sx >= 0 && sy >= 0 && gx >= 0 && gy >= 0);
  typedef pair<double, PI> PDPI;
  priority_queue<PDPI, vector<PDPI>, less<PDPI> > que;
  double inf = 1e16;
  vector<double> dists(n * m, 0);
  que.push(PDPI(inf, PI(gx, gy)));
  while (not que.empty()) {
    PDPI cur = que.top(); que.pop();
    int x = cur.second.first;
    int y = cur.second.second;
    if (dists[x * m + y] >= cur.first) { continue; }
    dists[x * m + y] = cur.first;
    if (sx == x && sy == y) { continue; }
    int dxy[5] = {0, 1, 0, -1, 0};
    REP(d, 0, 4) {
      int nx = x + dxy[d], ny = y + dxy[d + 1];
      if (nx < 0 || nx >= n || ny < 0 || ny >= m) { continue; }
      if (s[nx][ny] == '#') { continue; }
      int curpos = s[x][y] - '0';
      double dist = min(cur.first * 0.99, double(curpos));
      que.push(PDPI(dist, PI(nx, ny)));
    }
  }
  double res = dists[sx * m + sy];
  if (res == 0) {
    cout << -1 << endl;
  } else {
    printf("%.10f\n", res * 0.99);
  }
}
