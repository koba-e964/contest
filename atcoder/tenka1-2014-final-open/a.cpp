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

ll powmod(ll a, ll e) {
  ll sum = 1;
  ll cur = a;
  while (e > 0) {
    if (e % 2) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}

int dp[65][2437];
ll fact[12345];
ll invfact[12345];
void init(void) {
  fact[0] = 1;
  invfact[0] = 1;
  REP(i, 1, 12345) {
    fact[i] = fact[i - 1] * i % mod;
    invfact[i] = powmod(fact[i], mod - 2);
  }
}

ll calc(int h, int n, int w, bool zero) {
  REP(i, 0, h + 1) {
    REP(j, 0, w + 1) {
      dp[i][j] = 0;
    }
  }
  dp[0][0] = 1;
  REP(i, 0, h) {
    REP(j, 0, w + 1) {
      // dp[i + 1][j]
      ll tmp = 0;
      REP(k, 0, min(zero && i == 0 ? n - 1: n, j) + 1) {
	ll agno = dp[i][j - k];
	agno = agno * fact[j] % mod;
	agno = agno * invfact[k] % mod;
	agno = agno * invfact[j - k] % mod;
	tmp = (tmp + agno) % mod;
      }
      dp[i + 1][j] = tmp;
    }
  }
  return dp[h][w];
}


int main(void){
  int h, n, w;
  cin >> h >> n >> w;
  init();
  ll tmp = calc(h, n, w, false) - calc(h, n, w - 1, true) + mod;
  tmp %= mod;
  cout << tmp << endl;
}
