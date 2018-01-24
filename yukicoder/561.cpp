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

const ll inf = 5e15; // 5000chou

int main(void) {
  int n;
  ll d;
  cin >> n >> d;
  VL t(n), k(n);
  REP(i, 0, n) {
    cin >> t[i] >> k[i];
  }
  vector<VL> dp(n + 1, VL(2, -inf)); // 0: tokyo , 1: kyoto
  dp[0][0] = 0;
  REP(i, 0, n) {
    dp[i + 1][0] = max(dp[i][0], dp[i][1] - d) + t[i];
    dp[i + 1][1] = max(dp[i][1], dp[i][0] - d) + k[i];
  }
  cout << max(dp[n][0], dp[n][1]) << endl;
}
