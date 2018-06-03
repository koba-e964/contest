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

const int DEBUG = 0;

#define MOCK 0

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;
typedef pair<PL, ll> PLPL;

const int N = 2010;
ll dp[N][N];
ll h, w;
int n, m;

ll nuri(const vector<PLPL> &pool) {
  VL xco, yco;
  REP(i, 0, pool.size()) {
    ll x = pool[i].first.first;
    ll y = pool[i].first.second;
    xco.push_back(x);
    yco.push_back(y);
  }
  sort(xco.begin(), xco.end());
  xco.erase(unique(xco.begin(), xco.end()), xco.end());
  sort(yco.begin(), yco.end());
  yco.erase(unique(yco.begin(), yco.end()), yco.end());
  REP(i, 0, pool.size()) {
    ll x = pool[i].first.first;
    ll y = pool[i].first.second;
    x = lower_bound(xco.begin(), xco.end(), x) - xco.begin();
    y = lower_bound(yco.begin(), yco.end(), y) - yco.begin();
    dp[x][y] += pool[i].second;
  }
  REP(i, 0, N) {
    REP(j, 0, N) {
      if (i > 0) dp[i][j] += dp[i - 1][j];
      if (j > 0) dp[i][j] += dp[i][j - 1];
      if (i > 0 && j > 0) dp[i][j] -= dp[i - 1][j - 1];
    }
  }
  ll tot = 0;
  REP(i, 0, xco.size() - 1) {
    REP(j, 0, yco.size() - 1) {
      ll area = xco[i + 1] - xco[i];
      area *= yco[j + 1] - yco[j];
      if (dp[i][j] > 0) tot += area;
    }
  }
  if (DEBUG) {
    REP(i, 0, xco.size()) {
      REP(j, 0, yco.size()) {
        cerr << " " << dp[i][j];
      }
      cerr << endl;
    }
  }
  return tot;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
#if MOCK
  h = w = 1e9;
  n = m = 1000;
#else
  cin >> h >> w >> n >> m;
#endif
  vector<string> a(n);
#if MOCK
  REP(i, 0, n) a[i] = string(m, '#');
#else
  REP(i, 0, n) cin >> a[i];
#endif
  vector<PLPL> pool;
  REP(i, 0, n) {
    REP(j, 0, m) {
      if (a[i][j] == '#') {
        pool.push_back(PLPL(PL(i, j), 1));
        pool.push_back(PLPL(PL(i + h - n + 1, j), -1));
        pool.push_back(PLPL(PL(i, j + w - m + 1), -1));
        pool.push_back(PLPL(PL(i + h - n + 1, j + w - m + 1), 1));
      }
    }
  }
  cout << nuri(pool) << endl;
}
