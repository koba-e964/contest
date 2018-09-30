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

#if 0

const int N = 30;
int dp[N][N][N];
int main(void) {
  int k;
  cin >> k;
  REP(i, 1, N) {
    REP(j, 1, N) {
      REP(l, 1, N) {
        int wl = 1;
        REP(x, j, min(k, i) + 1) {
          wl &= dp[i - x][l][x];
        }
        dp[i][j][l] = 1 - wl;
      }
    }
  }
  REP(i, 0, N) {
    cerr << " " << dp[i][1][1];
    if (i % 10 == 9) cerr << endl;
  }
  cerr << endl;
}
#else

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k, m;
  cin >> n >> k >> m;
  VI a(n);
  REP(i, 0, n) cin >> a[i];
  REP(i, 0, m - 1) {
    int b, c;
    cin >> b >> c;
  }
  assert (n == 1 && m == 1);
  bool win = 0;
  if (a[0] <= k) {
    win = 1;
  } else {
    win = (k - a[0]) % 2 == 0;
  }
  cout << (win ? "Chinatsu" : "Akari") << endl;
}
#endif
