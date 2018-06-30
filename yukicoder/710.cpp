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

const int N = 100100;
int dp[N], dp2[N];
const int inf = 1e8;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI a(n), b(n);
  REP(i, 0, n) cin >> a[i] >> b[i];
  REP(i, 0, N) dp[i] = inf;
  dp[0] = 0;
  REP(i, 0, n) {
    REP(j, 0, N) dp2[j] = inf;
    REP(j, 0, N) {
      dp2[j] = min(dp2[j], dp[j] + b[i]);
    }
    REP(j, 0, N - a[i]) {
      dp2[j + a[i]] = min(dp2[j + a[i]], dp[j]);
    }
    swap(dp, dp2);
    if (0) {
      REP(j, 0, 100) {
        cerr << " " << dp[j];
      }
      cerr << endl;
    }
  }
  int mi = inf;
  REP(i, 0, N) {
    mi = min(mi, max(i, dp[i]));
  }
  cout << mi << endl;
}
