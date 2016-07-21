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
typedef pair<ll, ll> PI;
const double EPS=1e-9;

const int N = 210;
const int W = 1010 * 210;
ll v[N], w[N];

int n;
ll limw;

vector<PI> enumerate(int step, int end) {
  vector<PI> ret;
  for (int bits = 0; bits < end; bits += step) {
    ll totv = 0;
    ll totw = 0;
    REP(i, 0, n) {
      if (bits & (1 << i)) {
	totv += v[i];
	totw += w[i];
      }
    }
    ret.push_back(PI(totw, -totv));
  }
  sort(ret.begin(), ret.end());
  vector<PI> ret2;
  ll curv = 1;
  REP(i, 0, ret.size()) {
    if (ret[i].second < curv) {
      ret2.push_back(PI(ret[i].first, -ret[i].second));
    }
    curv = min(curv, ret[i].second);
  }
  return ret2;
}
void solve1() {
  ll ma = 0;
  vector<PI> former = enumerate(1, 1 << (n / 2));
  vector<PI> latter = enumerate(1 << (n / 2), 1 << n);
  REP(i, 0, latter.size()) {
    ll remw = limw - latter[i].first;
    if (remw < 0) {
      continue;
    }
    int p = upper_bound(former.begin(), former.end(), PI(remw, 0)) - former.begin();
    if (p == 0) {
      continue;
    }
    ma = max(ma, latter[i].second + former[p - 1].second);
  }
  cout << ma << endl;
}
ll dp[N][W];

void solve2() {
  //w <= 1000
  REP(j, 0, W) {
    dp[0][j] = 0;
  }
  REP(i, 0, n) {
    REP(j, 0, W) {
      dp[i + 1][j] = dp[i][j];
      if (j >= w[i])
	dp[i + 1][j] = max(dp[i + 1][j], dp[i][j - w[i]] + v[i]);
    }
  }
  ll ma = 0;
  REP(j, 0, limw + 1) {
    ma = max(ma, dp[n][j]);
  }
  cout << ma << endl;
}

void solve3() {
    ll inf = (1LL << 32) * N;
    REP(j, 0, W) {
        dp[0][j] = inf;
    }
    dp[0][0] = 0;
    REP(i, 0, n) {
        REP(j, 0, W) {
            dp[i + 1][j] = dp[i][j];
            if (j >= v[i]) {
                dp[i + 1][j] = min(dp[i + 1][j], dp[i][j - v[i]] + w[i]);
            }
        }
    }
    ll ma = 0;
    REP(j, 0, W) {
        if (dp[n][j] <= limw) {
            ma = j;
        }
    }
    cout << ma << endl;
    
}

int main(void){
    cin >> n >> limw;
    REP(i, 0, n) {
        cin >> v[i] >> w[i];
    }
    int w1000 = 1;
    REP(i, 0, n) {
        w1000 &= w[i] <= 1000;
    }
    int v1000 = 1;
    REP(i, 0, n) {
        v1000 &= v[i] <= 1000;
    }
    if (v1000) {
        solve3();
	return 0;
    }
    if (w1000) {
        solve2();
        return 0;
    }
    if (n <= 30) {
        solve1();
        return 0;
    }
    assert (0);
}
