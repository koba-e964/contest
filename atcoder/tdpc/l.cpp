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

const int inf = 1e8;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  vector<VI> f(n, VI(n));
  REP(i, 0, n) {
    REP(j, 0, n) {
      cin >> f[i][j];
    }
  }
  vector<VI> dp(n, VI(n, -inf));
  dp[0][0] = 0;
  REP(i, 1, n) {
    int acc = 0;
    REP(j, 0, i) {
      acc += f[j][i];
    }
    int ma = -inf;
    REP(k, 0, i + 1) {
      ma = max(ma, dp[i - 1][k]);
      dp[i][k] = ma + acc;
      acc -= f[k][i];
    }
  }
  int ma = -inf;
  REP(i, 0, n) {
    ma = max(ma, dp[n - 1][i]);
  }
  cout << 2 * ma << endl;
}
