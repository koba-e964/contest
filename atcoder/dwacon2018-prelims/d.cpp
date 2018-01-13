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
const int N = 20;


ll dp[1 << N];
int n;
int x[N], a[N];
int desc[N];

ll dfs(int bits) {
  if (bits == 0) {
    return 0;
  }
  if (dp[bits] >= 0) {
    return dp[bits];
  }
  ll mi = 1e16;
  REP(i, 0, n) {
    if (bits & 1 << i) {
      int up = bits | desc[i];
      ll upw = 0;
      REP(j, 0, n) {
	if (up & 1 << j) upw += x[j];
      }
      ll res = dfs((bits ^ 1 << i) | desc[i]);
      mi = min(mi, max(upw, res));
    }
  }
  dp[bits] = mi;
  return mi;
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n;
  a[0] = -1;
  REP(i, 0, n) {
    cin >> x[i];
  }
  REP(i, 1, n) {
    cin >> a[i];
    a[i]--;
    desc[a[i]] |= 1 << i;
  }
  REP(i, 0, 1 << N) {
    dp[i] = -1;
  }
  dfs(1);
  cout << dp[1] << endl;
}
