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

const int N = 1010;
ll dp[N][N];

void init(void){
  dp[0][0] = 1;
  REP(i, 1, N) {
    REP(j, 0, N) {
      ll tot = dp[i - 1][j];
      if (j >= i) {
	tot = (tot + dp[i][j - i]) % mod;
      }
      dp[i][j] = tot;
    }
  }
}

ll calc(int sc, int n, const VI &kill) {
  // run-length
  VI rl;
  int cur = -1;
  int cnt = 0;
  REP(i, 0, n) {
    if (cur != kill[i]) {
      if (cnt > 0) {
	rl.push_back(cnt);
      }
      cnt = 0;
    }
    cur = kill[i];
    cnt += 1;
  }
  if (cnt > 0) {
    rl.push_back(cnt);
  }
  int m = rl.size();
  vector<VL> tbl(m + 1, VL(N, 0));
  tbl[0][0] = 1;
  REP(i, 0, m) {
    REP(j, 0, N) {
      REP(k, 0, N - j) {
	tbl[i + 1][j + k] = (tbl[i + 1][j + k] + tbl[i][j] * dp[rl[i]][k]) % mod;
      }
    }
  }
  return tbl[m][sc];
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  init();
  int n, m;
  cin >> n >> m;
  VI a(n), b(m);
  int ta = 0, tb = 0;
  REP(i, 0, n) {
    cin >> a[i];
    ta += a[i];
  }
  REP(i, 0, m) {
    cin >> b[i];
    tb += b[i];
  }
  cout << calc(ta, m, b) * calc(tb, n, a) % mod << endl;
}
