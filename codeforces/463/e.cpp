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
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

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

void add(ll &x, ll y) {
  x = (x + y) % mod;
}

const int N = 5010;

ll dp[N][N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll n;
  int k;
  cin >> n >> k;
  REP(i, 0, k + 1) {
    dp[0][i] = powmod(2, n - k + i + mod - 1);
  }
  REP(i, 1, k + 1) {
    REP(j, i, k + 1) {
      ll tmp = dp[i - 1][j] - dp[i - 1][j - 1] + mod;
      tmp %= mod;
      tmp = tmp * (n - k + j + mod) % mod;
      dp[i][j] = tmp;
    }
  }
  cout << dp[k][k] << "\n";
}
