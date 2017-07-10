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

const ll inf = 5e15; // 5000-chou

const int N = 1234567;
ll fact[N];

void init(void) {
  fact[0] = 1;
  REP(i, 1, N) {
    fact[i] = fact[i - 1] * i % mod;
  }
}

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

ll comb(int x, int y) {
  if (y < 0 || y > x) {
    return 0;
  }
  return fact[x] * powmod(fact[y] * fact[x - y] % mod, mod - 2) % mod;
}

int main(void){
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k;
  ll l;
  cin >> n >> l >> k;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  VL dp(n * k, -inf);
  vector<PI> ops;
  REP(i, 0, n) {
    ops.push_back(PI(a[i], -(i + 1)));
    ops.push_back(PI(a[i], i + 1));
  }
  sort(ops.begin(), ops.end());
  REP(i, 0, n) {
    dp[i] = 1;
  }
  REP(i, 1, k) {
    ll cur = 0;
    REP(opidx, 0, ops.size()) {
      PI op = ops[opidx];
      if (op.second <= -1) { // update
	int idx = -op.second - 1;
	cur = (cur + dp[(i - 1) * n + idx]) % mod;
      } else {
	int idx = op.second - 1;
	dp[i * n + idx] = cur;
      }
    }
    if (0) {
      REP(l, 0, n) {
	cerr << "dp[" << i << " " << l << "] = " << dp[i * n + l] << endl;
      }
    }
  }
  ll sum = 0;
  REP(i, 0, k) {
    REP(j, 0, n) {
      ll freedom = j + 1 <= l ? (l - j - 1) / n + 1 : 0;
      ll ctbl = 0;
      if (freedom > 0) {
	ctbl = freedom >= i ? (freedom + mod - i) % mod : 0;
      }
      // C(freedom, i + 1)
      sum += dp[i * n + j] * ctbl % mod;
      sum %= mod;
    }
  }
  cout << sum << endl;
}
