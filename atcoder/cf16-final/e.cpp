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

ll solve(ll  n, ll a) {
  if (n <= 1) {
    return 1;
  }
  VL dp(n, -1);
  for (int i = n - 1; i >= 1; --i) {
    ll res = (n + i - 1) / i;
    REP(j, 2, n / i + 2) {
      ll idx = i * j;
      res = min(res, j + a + (idx >= n ? 1 : dp[idx]));
    }
    dp[i] = res;
  }
  if (0) {
    REP(i, 0, n) {
      cout << "dp[" << i << "]=" << dp[i] << endl;
    }
  }
  return dp[1];
}

int main(void){
  ll n, a;
  cin >> n >> a;
  if (1) {
    cerr << "solve(-, 10):";
    int cur = 0;
    REP(i, 1, 1000) {
      int w = solve(i, 10);
      if (cur < w) {
	cerr << "solve(" << i << ",1)" << w << endl;
      }
      cur = w;
    }
    cerr << endl;
  }
  cout << solve(n, a) << endl;
}
