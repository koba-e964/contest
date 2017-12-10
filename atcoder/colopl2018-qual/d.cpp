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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 5010;

ll dp[N][N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  ll x;
  cin >> n >> x;
  VL t(n);
  VI prev(n);
  REP(i, 0, n) {
    cin >> t[i];
    prev[i] = upper_bound(t.begin(), t.begin() + i, t[i] - x) - t.begin();
  }
  REP(i, 0, n) {
    dp[1][i] = x;
  }
  REP(k, 2, n + 1) {
    REP(i, 0, n) {
      int p = prev[i];
      ll ma = 0;
      if (p > 0) {
	ma = max(ma, dp[k - 1][p - 1] + x);
      }
      if (p < i) {
	ma = max(ma, dp[k - 1][p] + t[i] - t[p]);
      }
      dp[k][i] = ma;
    }
  }
  VL ret(n + 1);
  REP(k, 1, n + 1) {
    REP(i, 0, n) {
      ret[k] = max(ret[k], dp[k][i]);
    }
    if (k >= 2) {
      ret[k] = max(ret[k], ret[k - 1]);
    }
  }
  REP(i, 1, n + 1) {
    cout << ret[i] << endl;
  }
}
