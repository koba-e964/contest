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

const int N = 5124356;

ll minpr[N];

ll dp[N];

int main(void){
  ll t;
  int l, r;
  cin >> t >> l >> r;
  REP(i, 0, N) {
    minpr[i] = i;
  }
  REP(i, 2, 2500) {
    if (minpr[i] != i) { continue; }
    REP(j, 2, (N - 1) / i + 1) {
      int k = i * j;
      if (minpr[k] == k) {
	minpr[k] = i;
      }
    }
  }
  dp[1] = 0;
  REP(i, 2, N) {
    ll to = i / minpr[i];
    ll tmp = dp[to];
    tmp += to * minpr[i] * (minpr[i] - 1) / 2;
    dp[i] = tmp;
  }
  ll tot = 0;
  ll cur = 1;
  REP(i, l, r + 1) {
    tot += cur * (dp[i] % mod) % mod;
    tot %= mod;
    cur = cur * t % mod;
  }
  cout << tot << endl;
}
