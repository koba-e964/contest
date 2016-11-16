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

const int N = 2016;
ll dp[N][N];
const ll inf = 1e10;

int main(void){
  int n;
  ll k;
  cin >> n >> k;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  int tot = 0;
  REP(i, 0, n) {
    tot += a[i];
  }
  if (tot == k) {
    cout << 1 << endl;
    return 0;
  }
  REP(i, 0, N) {
    REP(j, 0, N) {
      dp[i][j] = inf;
    }
  }
  dp[0][0] = 0;
  tot = 0;
  REP(i, 0, n) {
    REP(j, 0, i + 1) {
      dp[i + 1][j] = min(dp[i + 1][j], dp[i][j]); // 0 wins
      if (j < n) {
	int add = tot == 0 ? 1 : dp[i][j] * a[i] / tot + 1;
	if (add <= a[i]) {
	  dp[i + 1][j + 1] = min(dp[i + 1][j + 1], dp[i][j] + add);
	}
      }
    }
#if 0
    cerr << "dp[" << i + 1 << "]:";
    REP(j, 0, n + 1) {
      if (dp[i + 1][j] == inf) {
	cerr << " inf";
      } else {
	cerr << " " << dp[i + 1][j];
      }
    }
    cerr << endl;
#endif
    tot += a[i];
  }
  int ma = 0;
  REP(i, 0, n + 1) {
    if (dp[n][i] <= k) {
      ma = i;
    }
  }
  cout << ma << endl;
}
