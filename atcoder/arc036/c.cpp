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
ll dp[N][N][N];


int main(void){
  int n, k;
  cin >> n >> k;
  string s;
  cin >> s;
  dp[0][0][0] = 1;
  REP(i, 1, n + 1) {
    REP(j, 0, k + 1) {
      REP(l, 0, k + 1) {
	if (s[i - 1] != '0') {
	  if (j <= k - 1) {
	    dp[i][j + 1][max(l - 1, 0)] += dp[i - 1][j][l];
	    dp[i][j + 1][max(l - 1, 0)] %= mod;
	  }
	}
	if (s[i - 1] != '1') {
	  if (l <= k - 1) {
	    dp[i][max(j - 1, 0)][l + 1] += dp[i - 1][j][l];
	    dp[i][max(j - 1, 0)][l + 1] %= mod;
	  }
	}
      }
    }
  }
  ll sum = 0;
  REP(i, 0, k + 1) {
    REP(l, 0, k + 1) {
      sum += dp[n][i][l];
      sum %= mod;
    }
  }
  cout << sum << endl;
}
