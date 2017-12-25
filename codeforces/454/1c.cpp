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

const int N = 22;
int gr[N];

// Reference: http://codeforces.com/contest/906/submission/33555026
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  REP(i, 0, m) {
    int u, v;
    cin >> u >> v;
    u--, v--;
    gr[u] |= 1 << v;
    gr[v] |= 1 << u;
  }
  if (2 * m == n * (n - 1)) {
    cout << 0 << "\n";
    return 0;
  }
  REP(i, 0, n) {
    gr[i] |= 1 << i;
  }
  VI dp(1 << n, 1e7);
  VI prev(1 << n, -1);
  VI pick(1 << n, -1);
  REP(i, 0, n) {
    dp[1 << i] = 0;
  }
  REP(bits, 1, 1 << n) {
    REP(i, 0, n) {
      if ((bits & 1 << i) == 0) { continue; }
      int nxt = bits | gr[i];
      if (dp[bits] + 1 < dp[nxt]) {
	dp[nxt] = dp[bits] + 1;
	prev[nxt] = bits;
	pick[nxt] = i;
      }
    }
  }
  int mi = dp[(1 << n) - 1];
  cout << mi << "\n";
  VI path;
  int cur = (1 << n) - 1;
  int rem = mi;
  while (rem > 0) {
    path.push_back(pick[cur]);
    cur = prev[cur];
    rem--;
  }
  reverse(path.begin(), path.end());
  REP(i, 0, path.size()) {
    cout << path[i] + 1 << (i == (int)path.size() - 1 ? "\n" : " ");
  }
}
