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
#include <utility>
#include <vector>

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const int N = 2010;
int n, m;
ll dp[N][N];
ll a[N];
ll b[N];
const ll inf = 1e16;

// dp[i][k] where k in [l, r) is computed.
// mini[i][k] is guaranteed to be in [lidx, ridx).
void compute(int i, int l, int r, int lidx, int ridx) {
  if (l >= r) return;
  int mid = (l + r) / 2;
  // Calculate dp[i][mid]
  int mini = -1;
  ll mi = inf;
  REP(k, lidx, min(ridx, mid)) {
    ll val = dp[i - 1][k] + (mid - k) * (b[mid] - b[k]);
    if (mi > val) {
      mi = val;
      mini = k;
    }
  }
  if (DEBUG) {
    cerr << "mini[" << i << "][" << mid << "]=" << mini << endl;
  }
  dp[i][mid] = mi;
  compute(i, l, mid, lidx, mini + 1);
  compute(i, mid + 1, r, mini, ridx);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n >> m;
  REP(i, 0, n) cin >> a[i];
  sort(a, a + n);
  REP(i, 0, n) b[i + 1] = b[i] + a[i];
  REP(i, 1, n + 1) dp[0][i] = inf;
  REP(i, 0, m) {
    compute(i + 1, 1, n + 1, 0, n);
    if (DEBUG) {
      REP(j, 0, n + 1) {
        cerr << " " << dp[i + 1][j];
      }
      cerr << endl;
    }
  }
  cout << dp[m][n] << endl;
}
