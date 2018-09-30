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
const ll mod = 1e9 + 7;


const int H = 13;
int dp[H][H];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int h, w;
  cin >> h >> w;
  vector<string> c(h);
  REP(i, 0, h) cin >> c[i];
  int sy = -1;
  REP(i, 0, w) {
    if (c[h - 1][i] == 's') sy = i;
  }
  dp[h - 1][sy] = 1;
  for (int i = h - 1; i >= 1; --i) {
    REP(y, 0, w) {
      if (dp[i][y] == 0) continue;
      for (int dy = -1; dy <= 1; ++dy) {
        int ny = y + dy;
        if (ny < 0 || ny >= w) continue;
        if (c[i - 1][ny] != 'x') {
          dp[i - 1][ny] = 1;
        }
      }
    }
  }
  if (0) {
    REP(i, 0, h) {
      REP(j, 0, w) cerr << " " << dp[i][j];
      cerr << endl;
    }
  }
  int ok = -1;
  REP(i, 0, w) {
    if (dp[0][i]) ok = i;
  }
  if (ok == -1) {
    cout << "impossible" << endl;
    return 0;
  }
  int cur = ok;
  string ops;
  REP(i, 0, h - 1) {
    int nxt = -1;
    char op = '+';
    REP(dy, -1, 2) {
      int ny = cur + dy;
      if (ny < 0 || ny >= w) continue;
      if (dp[i + 1][ny]) {
        nxt = ny;
        op = "RSL"[dy + 1];
        break;
      }
    }
    cur = nxt;
    ops += op;
  }
  reverse(ops.begin(), ops.end());
  cout << ops << endl;
}
