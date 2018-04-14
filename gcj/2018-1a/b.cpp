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

ll solve(int r, ll b, int c, const VL &m, const VL &s, const VL &p) {
  ll pass = 1.1e18;
  ll fail = 0;
  while (pass - fail > 1) {
    ll mid = (pass + fail) / 2;
    VL pool(c);
    REP(i, 0, c) {
      pool[i] = mid >= p[i] ? min(m[i], (mid - p[i]) / s[i]) : 0;
    }
    sort(pool.rbegin(), pool.rend());
    ll sum = 0;
    REP(i, 0, r) sum += pool[i];
    if (sum >= b) {
      pass = mid;
    } else {
      fail = mid;
    }
  }
  return pass;
}

const ll inf = 1e18;

ll naive(int r, ll b, int c, const VL &m, const VL &s, const VL &p) {
  vector<VL> dp(r + 1, VL(b + 1, inf));
  dp[0][0] = 0;
  REP(i, 0, c) {
    vector<VL> dp2 = dp;
    int lim = min(b, m[i]);
    REP(j, 1, lim + 1) {
      REP(k, 0, b - j + 1) {
	REP(l, 0, r) {
	  dp2[l + 1][k + j] = min(dp2[l + 1][k + j], max(dp[l][k], p[i] + j * s[i]));
	}
      }
    }
    swap(dp, dp2);
    if (0) {
      cerr << "dp[" << i + 1 << "]:";
      REP(j, 0, r + 1) {
	REP(k, 0, b + 1) {
	  cerr << " " << dp[j][k];
	}
	cerr << endl;
      }
    }
  }
  ll mi = inf;
  REP(i, 0, r + 1) {
    mi = min(mi, dp[i][b]);
  }
  return mi;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  REP(case_nr, 1, t + 1) {
    int r, c;
    ll b;
    cin >> r >> b >> c;
    VL m(c), s(c), p(c);
    REP(i, 0, c) {
      cin >> m[i] >> s[i] >> p[i];
    }
    ll ans = solve(r, b, c, m, s, p);
    if (0) {
      ll ans_naive = naive(r, b, c, m, s, p);
      if (ans != ans_naive) {
	cerr << "ERROR! naive:" << ans_naive << " solve: " << ans << endl;
      }
    }
    cout << "Case #" << case_nr << ": " << ans << endl;
  }
}
