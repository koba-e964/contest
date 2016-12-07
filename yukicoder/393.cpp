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

const int M = 61;
const int W = 100100;
int dp[M][W];

int solve(int n1, int n2, const VI &a) {
  int m = a.size();
  REP(i, 0, W) {
    dp[0][i] = 0;
  }
  dp[0][0] = 1;
  int tot = 0;
  REP(i, 0, m) {
    tot += a[i];
    REP(j, 0, W) { dp[i + 1][j] = dp[i][j]; }
    REP(j, 0, W) {
      if (j >= a[i]) {
	dp[i + 1][j] |= dp[i][j - a[i]];
      }
    }
    bool ok = 0;
    REP(j, 0, n1 + 1) {
      if (dp[i + 1][j] && tot - j <= n2) {
	ok = 1;
      }
    }
    if (not ok) {
      return i;
    }
  }
  return m;
}

int main(void){
  int d;
  cin >> d;
  while(d--) {
    int n1, n2;
    cin >> n1 >> n2;
    int m;
    cin >> m;
    VI a(m);
    REP(i, 0, m) { cin >> a[i]; }
    sort(a.begin(), a.end());
    cout << solve(n1, n2, a) << endl;
  }
}
