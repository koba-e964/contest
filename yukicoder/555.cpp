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

const int inf = 1e8;

int main(void) {
  int n;
  cin >> n;
  int c, v;
  cin >> c >> v;
  VI dp(n + 1, inf);
  dp[1] = 0;
  REP(i, 1, n) {
    for (int j = 2; i * j <= n; ++j) {
      dp[i * j] = min(dp[i * j], dp[i] + c + (j - 1) * v);
    }
    dp[n] = min(dp[n], dp[i] + c + ((n + i - 1) / i - 1) * v);
  }
  cout << dp[n] << endl;
}
