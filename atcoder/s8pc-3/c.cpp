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

const int N = 101;
const int K = 256;
ll fact[N];



ll dp[N][N][K];

int main(void){
  int n, k;
  cin >> n >> k;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  fact[0] = 1;
  REP(i, 1, N) {
    fact[i] = fact[i - 1] * i % mod;
  }
  dp[0][0][0] = 1;
  REP(i, 0, n) {
    REP(j, 0, i + 1) {
      REP(l, 0, K) {
	dp[i + 1][j][l] += dp[i][j][l];
	dp[i + 1][j][l] %= mod;
	dp[i + 1][j + 1][l] += dp[i][j][l ^ a[i]];
	dp[i + 1][j + 1][l] %= mod;
      }
    }
  }
  
  ll cnt = 0;
  REP(i, 0, n + 1) {
    cnt += dp[n][i][k] * fact[i] % mod;
    cnt %= mod;
  }
  cout << cnt << endl;
}
