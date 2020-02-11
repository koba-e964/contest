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

const int N = 100010;
const int W = 1000010;
int a[N];
VI occ[W];
int dp[N][2];

// G
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 0, n) cin >> a[i];
  REP(i, 0, n) {
    occ[a[i]].push_back(i);
  }
  REP(i, 0, n) {
    dp[i][0] = 1;
    dp[i][1] = 1;
    int v = a[i];
    int idx = upper_bound(occ[v - 1].begin(), occ[v - 1].end(), i) - occ[v - 1].begin();
    if (idx > 0) {
      idx = occ[v - 1][idx - 1];
      REP(j, 0, 2) {
        dp[i][j] = max(dp[i][j], dp[idx][j] + 1);
      }
    }
    if (v >= 2) {
      idx = upper_bound(occ[v - 2].begin(), occ[v - 2].end(), i) - occ[v - 2].begin();
      if (idx > 0) {
        idx = occ[v - 2][idx - 1];
        dp[i][1] = max(dp[i][1], dp[idx][0] + 2);
      }
    }
  }
  int ma = 0;
  REP(i, 0, n) ma = max(ma, max(dp[i][0] + 1, dp[i][1]));
  cout << ma << endl;
}
