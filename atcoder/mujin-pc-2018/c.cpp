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

const int N = 2100;
int dp[4][N][N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  vector<string> s(n);
  REP(i, 0, n) cin >> s[i];
  REP(i, 0, n) {
    int cnt = 0;
    REP(j, 0, m) {
      if (s[i][j] == '#') {
	cnt = 0;
      } else {
	cnt++;
      }
      dp[0][i][j] = cnt;
    }
    cnt = 0;
    for (int j = m - 1; j >= 0; --j) {
      if (s[i][j] == '#') {
	cnt = 0;
      } else {
	cnt++;
      }
      dp[2][i][j] = cnt;
    }
  }
  REP(j, 0, m) {
    int cnt = 0;
    REP(i, 0, n) {
      if (s[i][j] == '#') {
	cnt = 0;
      } else {
	cnt++;
      }
      dp[1][i][j] = cnt;
    }
    cnt = 0;
    for (int i = n - 1; i >= 0; --i) {
      if (s[i][j] == '#') {
	cnt = 0;
      } else {
	cnt++;
      }
      dp[3][i][j] = cnt;
    }
  }
  if (DEBUG) {
    REP(d, 0, 4) {
      cerr << "dp[" << d << "]:" << endl;
      REP(i, 0, n) {
	REP(j, 0, m) {
	  cerr << " " << dp[d][i][j];
	}
	cerr << endl;
      }
    }
  }
  ll tot = 0;
  REP(i, 0, n) {
    REP(j, 0, m) {
      if (s[i][j] == '#') continue;
      REP(d, 0, 4) {
	tot += (dp[d][i][j] - 1) * (dp[(d + 1) % 4][i][j] - 1);
      }
    }
  }
  cout << tot << endl;
}
