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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 21;
ll fact[N];
void init(void) {
  fact[0] = 1;
  REP(i, 1, N) {
    fact[i] = fact[i - 1] * i % mod;
  }
}

int main(void) {
  init();
  int n;
  cin >> n;
  vector<string> s(n);
  REP(i, 0, n) {
    cin >> s[i];
    s[i] += '$';
  }
  vector<VI> lcp(n, VI(n));
  REP(i, 0, n) {
    REP(j, 0, n) {
      int idx = 0;
      while (true) {
	if ((int) s[i].length() <= idx || (int) s[j].length() <= idx) {
	  break;
	}
	if (s[i][idx] == s[j][idx]) {
	  idx += 1;
	} else {
	  break;
	}
      }
      lcp[i][j] = idx;
    }
  }
  vector<ll> dp(1 << n);
  dp[0] = 0;
  REP(bits, 1, 1 << n) {
    int k = __builtin_popcount(bits) - 1;
    ll fac = fact[k];
    ll tot = 0;
    vector<int> remaining;
    REP(i, 0, n) {
      if ((bits & 1 << i) == 0) {
	remaining.push_back(i);
      }
    }
    REP(i, 0, n) {
      if ((bits & 1 << i) == 0) {
	continue;
      }
      int idx = 1;
      REP(j, 0, remaining.size()) {
	int jidx = remaining[j];
	idx = max(idx, lcp[jidx][i] + 1);
      }
      // cerr << "tt" << bits << " " << i << " " << idx << endl;
      tot += dp[bits ^ 1 << i] + idx * fac % mod;
    }
    dp[bits] = tot % mod;
  }
  VL tbl(n + 1, 0);
  REP(bits, 0, 1 << n) {
    tbl[__builtin_popcount(bits)] += dp[bits];
  }
  REP(i, 1, n + 1) {
    cout << tbl[i] % mod << endl;
  }
}
