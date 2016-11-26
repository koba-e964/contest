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

const int N = 310;

ll fact[N];


ll dp[N][N][N];
ll ppow[N];

int main(void){
  int n, m;
  cin >> n >> m;
  if (n > m) {
    cout << 0 << endl;
    return 0;
  }
  fact[0] = 1;
  REP(i, 1, N) {
    fact[i] = fact[i - 1] * i % mod;
  }
  dp[0][1][0] = 1;
  REP(i, 1, m + 1) {
    REP(j, 1, n + 1) {
      REP(k, 0, j) {
	if (j >= 1) {
	  dp[i][j][k] += dp[i - 1][j- 1][k];
	}
	dp[i][j][k] %= mod;
	dp[i][j][k] += dp[i - 1][j][k] * (j - k - 1);
	dp[i][j][k] %= mod;
	dp[i][j][j - 1] += dp[i - 1][j][k] * (k + 1);
	dp[i][j][j - 1] %= mod;
      }
      if (0) {
	cerr << "dp[" << i << "][" << j << "]:";
	REP(k, 0, j) {
	  cerr << dp[i][j][k] << " ";
	}
	cerr << endl;
      }
    }

  }
  ll tot = 0;
  ppow[0] = 1;
  REP(i, 1, N) {
    ppow[i] = ppow[i - 1] * (n - 1) % mod;
  }
  cout << dp[m][n][n -1] * fact[n - 1] % mod << endl;
}
