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

const int N = 501;

int n, m, q;
string a[N];
string s;

const int B = 30;

PI dbl[N][N][B];

PI naive(int x, int y, int cnt) {
  REP(i, 0, cnt) {
    int dx = s[i] == 'U' ? -1 : s[i] == 'D' ? 1 : 0;
    int dy = s[i] == 'L' ? -1 : s[i] == 'R' ? 1 : 0;
    int nx = x + dx, ny = y + dy;
    if (nx < 0 || nx >= n || ny < 0 || ny >= m) {
      continue;
    }
    if (a[nx][ny] != '.') continue;
    x = nx; y = ny;
  }
  return PI(x, y);
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n >> m >> q;
  REP(i, 0, n) cin >> a[i];
  cin >> s;
  REP(i, 0, n) {
    REP(j, 0, m) {
      PI ans = naive(i, j, s.size());
      dbl[i][j][0] = ans;
    }
  }
  REP(b, 0, B - 1) {
    REP(i, 0, n) {
      REP(j, 0, m) {
        PI t = dbl[i][j][b];
        PI u = dbl[t.first][t.second][b];
        dbl[i][j][b + 1] = u;
      }
    }
  }
  REP(i, 0, q) {
    int x, y;
    ll z;
    cin >> x >> y >> z;
    x--, y--;
    ll q = z / s.size();
    for (int b = B - 1; b >= 0; b--) {
      if (q >= 1LL << b) {
        PI t = dbl[x][y][b];
        x = t.first, y = t.second;
        q -= 1LL << b;
      }
    }
    PI u = naive(x, y, z % s.size());
    cout << u.first + 1 <<  " " << u.second + 1 << endl;
  }
}
