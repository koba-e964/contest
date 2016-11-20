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
const ll mod = 998244353;

ll invmod(ll x) {
  ll sum = 1;
  ll cur = x;
  ll e = mod - 2;
  while (e > 0) {
    if (e % 2 == 1) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}


const int N = 3100;
ll dp[N][N];

void solve_dp(int n, int m) {
  dp[1][0] = 0;
  dp[1][1] = 1;
  REP(i, 2, m + 1) {
    dp[1][i] = (dp[1][i - 1] + dp[1][i - 2]) % mod;
  }
  REP(i, 2, n + 1) {
    REP(j, 1, m + 1) {
      dp[i][j] = (dp[i - 1][j] + dp[i][j - 1]) % mod;
    }
  }
  cout << dp[n][m] << endl;
}

vector<VL> mul(const vector<VL> &a, const vector<VL> &b) {
  vector<VL> res(2, VL(2));
  REP(i, 0, 2) {
    REP(j, 0, 2) {
      REP(k, 0, 2) {
	res[i][j] += a[i][k] * b[k][j];
	res[i][j] %= mod;
      }
    }
  }
  return res;
}

vector<VL> pow(const vector<VL> &a, ll e) {
  vector<VL> sum(2, VL(2));
  REP(i, 0, 2) {
    sum[i][i] = 1;
  }
  vector<VL> cur = a;
  while (e > 0) {
    if (e % 2 == 1) {
      sum = mul(sum, cur);
    }
    cur = mul(cur, cur);
    e /= 2;
  }
  return sum;
}

int main(void){
  int n;
  ll m;
  cin >> n >> m;
  // matrix
  vector<VL> a(2, VL(2, 1));
  a[0][0] = 0;
  vector<VL> a_2m_2 = pow(a, m + 2 * n - 2);
  VL binom(n - 1, 1);
  REP(i, 1, n - 1) {
    ll tmp = binom[i - 1] * ((m + i) % mod) % mod;
    tmp = tmp * invmod(i) % mod;
    binom[i] = tmp;
  }
  ll sum = a_2m_2[0][1];
  vector<VL> cur = a;
  vector<VL> a2 = mul(a, a);
  REP(i, 0, n - 1) {
    ll tmp = binom[n - 2 - i] * cur[0][1] % mod;
    sum = (sum - tmp + mod) % mod;
    cur = mul(cur, a2);
  }
  cout << sum << endl;
  return 0;
}
