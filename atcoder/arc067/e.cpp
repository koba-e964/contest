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

const int DEBUG = 0;
const int N = 1010;
ll dp[N][N];
ll fact[N];
// dp[k]:= the number of way to do in the first k persons
int main(void){
  int n, a, b, c, d;
  cin >> n >> a >> b >> c >> d;
  fact[0] = 1;
  REP(i, 1, N) {
    fact[i] = fact[i - 1] * i % mod;
  }
  REP(i, a - 1, b) {
    dp[0][i] = 1;
  }
  REP(i, 1, n + 1) {
    dp[i][a - 1] = 0;
    REP(j, a - 1, b) {
      ll sum = 0;
      int u = j + 1;
      REP(f, c, d + 1) {
	if (u * f <= i) {
	  ll tmp = dp[i - u * f][u - 1];
	  // divide by (u!)^f * f!
	  ll tmp2 = fact[f];
	  tmp2 = tmp2 * fact[i - u * f] % mod;
	  // TLE
	  tmp2 = tmp2 * powmod(fact[u], f) % mod;
	  tmp2 = fact[i] * powmod(tmp2, mod - 2) % mod;
	  tmp = tmp * tmp2 % mod;
	  if (DEBUG) {
	    cout << "(u, f)=" << u << ", " << f << endl;
	    cout << "tmp=" << tmp << endl;
	    cout << "tmp2=" << tmp2 << endl;
	  }
	  sum += tmp;
	}
      }
      sum += dp[i][j];
      sum %= mod;
      dp[i][u] = sum;
      if (DEBUG) {
	cerr << "dp[" << i  << "," << u << "]=" << dp[i][u] << endl;
      }
    }
  }
  ll tmp = 0;
  cout << dp[n][b] << endl;
}
