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
void add(ll &x, ll y) {
  x = (x + y) % mod;
}


const int N = 1 << 20;
ll dp[N];

ll p2[N];

void init(void) {
  p2[0] = 1;
  REP(i, 1, N) {
    p2[i] = p2[i - 1] * 2 % mod;
  }
}

int main(void) {
  init();
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
    dp[a[i]]++;
  }
  REP(i, 0, 20) {
    REP(bits, 0, N) {
      if (bits & 1 << i) {
	dp[bits ^ 1 << i] += dp[bits];
      }
    }
  }
  REP(bits, 0, N) {
    dp[bits] = p2[dp[bits]];
  }
  ll tot = 0;
  REP(bits, 0, N) {
    ll tmp = dp[bits];
    if (__builtin_popcount(bits) % 2 == 1) {
      tmp = mod - tmp;
    }
    add(tot, tmp);
  }
  cout << tot << endl;
}
