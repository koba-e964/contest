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

const int N = 72;

ll dp[N][N];
ll dp2[N][N];
void add(ll &x, ll y) {
  x = (x + y) % mod;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  dp[0][0] = 1;
  REP(k, 1, N) {
    memset(dp2, 0, sizeof(dp2));
    REP(i, 0, N) {
      REP(j, 0, N) {
	REP(l, 0, N) {
	  if (j + k * l >= N || i + l >= N) continue;
	  add(dp2[i+l][j+k*l], dp[i][j]);
	}
      }
    }
    swap(dp, dp2);
  }
  ll tot = 0;
  REP(i, 0, n + 1) {
    REP(j, 1, n + 1) {
      if (i + j - 1 > n) { continue; }
      add(tot, dp[j][i]);
    }
  }
  cout << tot << endl;
}
