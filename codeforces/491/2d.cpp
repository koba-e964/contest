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
const ll mod = 1e9 + 7;

const int N = 110;
int dp[N][4];
const int inf = 1e8;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  vector<string> b(2);
  REP(i, 0, 2) cin >> b[i];
  int n = b[0].size();
  VI pat(n);
  REP(i, 0, n) {
    REP(j, 0, 2) {
      if (b[j][i] == 'X') pat[i] |= 1 << j;
    }
  }
  REP(i, 0, n + 1) {
    REP(j, 0, 4) {
      dp[i][j] = -inf;
    }
  }
  dp[0][3] = 0;
  int sigma[4][2] = {{3, 1}, {3, 2}, {1, 3}, {2, 3}};
  REP(i, 0, n) {
    REP(b, 0, 4) {
      REP(c, 0, 4) {
        for (auto s: sigma) {
          if ((b & s[0]) != 0 || (c & s[1]) != 0) continue;
          if ((c & pat[i]) != 0 || (s[1] & pat[i]) != 0) continue;
          int nxt = c | s[1] | pat[i];
          dp[i + 1][nxt] = max(dp[i + 1][nxt], dp[i][b] + 1);
        }
      }
      dp[i + 1][pat[i]] = max(dp[i + 1][pat[i]], dp[i][b]);
    }
  }
  if (DEBUG) {
    REP(i, 0, n + 1) {
      REP(j, 0, 4) {
        cerr << " " << dp[i][j];
      }
      cerr << endl;
    }
  }
  int ma = -inf;
  REP(i, 0, 4) ma = max(ma, dp[n][i]);
  cout << ma << "\n";
}
