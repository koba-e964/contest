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

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;

const ll inf = 1e17;

void chmin(ll &x, ll y) {
  x = min(x, y);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL a(n), b(n);
  REP(i, 0, n) cin >> a[i];
  REP(i, 0, n) cin >> b[i];
  ll pass = 2.5e12, fail = -1;
  bool disp = false;
  while (pass - fail > 1) {
    ll mid = (pass + fail) / 2;
    map<ll, VL> noel;
    REP(i, 0, n) {
      ll fiat = a[i] * 2000 - b[i] * (2 * mid);
      noel[a[i]].push_back(fiat);
    }
    vector<pair<ll, VL> > noel_v(noel.begin(), noel.end());
    sort(noel_v.rbegin(), noel_v.rend());
    int m = noel_v.size();
    REP(i, 0, m) {
      sort(noel_v[i].second.begin(), noel_v[i].second.end());
    }
    vector<VL> dp(m + 1, VL(n + 1, inf));
    dp[0][0] = 0;
    REP(i, 0, m) {
      int k = noel_v[i].second.size();
      const VL &wei = noel_v[i].second;
      ll tap = 0;
      if (DEBUG && not disp) {
        DEBUGP(noel_v[i].first);
        cerr << "wei:" << endl;
        for (auto w: wei) cerr << " " << w;
        cerr << endl;
      }
      REP(l, 0, k + 1) { // l firsts, (k - l) seconds
        REP(j, k - l, n + 1) {
          int to = j + l - (k - l);
          if (0 <= to && to <= n) {
            chmin(dp[i + 1][to], dp[i][j] + tap);
          }
        }
        if (l < k) {
          tap += wei[l];
        }
      }
    }
    disp = true;
    if (DEBUG && mid <= 0) {
      DEBUGP(mid);
      cerr << "dp:" << endl;
      REP(i, 0, m + 1) {
        REP(j, 0, n + 1) cerr << " " << dp[i][j];
        cerr << endl;
      }
    }
    ll mi = inf;
    REP(i, 0, n + 1) mi = min(mi, dp[m][i]);
    bool ok = mi <= 0;
    if (ok) {
      pass = mid;
    } else {
      fail = mid;
    }
  }
  cout << pass << "\n";
}
