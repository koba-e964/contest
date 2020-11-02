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

double dp[11][21][401];
double acc[11][21][401];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  REP(f, 1, 21) {
    dp[0][f][0] = 1;
    REP(i, 0, 10) {
      REP(x, 1, f + 1) {
        REP(k, 0, 401 - x) {
          dp[i + 1][f][k + x] += dp[i][f][k] / f;
        }
      }
    }
    REP(i, 0, 11) {
      for (int j = 400 - 1; j >= 0; j--) {
        dp[i][f][j] += dp[i][f][j + 1];
      }
    }
  }
  int t;
  cin >> t;
  while (t--) {
    int w, d;
    cin >> w >> d;
    double ma = 0;
    REP(i, 0, w) {
      int n, f, m;
      cin >> n >> f >> m;
      int lim = d - m;
      double prob = 0.0;
      if (lim > 400) {
        prob = 0.0;
      } else if (lim < 0) {
        prob = 1.0;
      } else {
        prob = dp[n][f][lim];
      }
      ma = max(ma, prob);
    }
    cout << fixed << setprecision(15) << ma << endl;
  }
}
