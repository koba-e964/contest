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

const int P = 80000;


void chmax(double &x, double y) {
  x = max(x, y);
}

double solve(int n, int p, const VI &w, const VI &h) {
  int perim = 0;
  REP(i, 0, n) {
    perim += 2 * (w[i] + h[i]);
  }
  p -= perim;
  vector<vector<double> > dp(n + 1, vector<double>(P + 1, -1));
  dp[0][0] = 0;
  REP(i, 0, n) {
    int low = 2 * min(w[i], h[i]);
    double hi = 2 * sqrt(w[i] * w[i] + h[i] * h[i]);
    REP(j, 0, P + 1) {
      if (dp[i][j] < 0) continue;
      chmax(dp[i + 1][j], dp[i][j]);
      for (int k = low; k <= low; ++k) {
	double margin = hi - low;
	if (j + k > P) continue;
        chmax(dp[i + 1][j + k], dp[i][j] + margin);
      }
    }
    if (0) {
      cerr << "dp[" << i + 1 << "]:";
      REP(j, 0, 10) {
	cerr << " " << dp[i+1][j];
      }
      cerr<<endl;
    }
  }
  pair<double, double> mi(1e8, -1);
  REP(i, 0, P + 1) {
    double val = dp[n][i];
    if (val < 0) continue;
    double lim;
    if (i <= p) {
      lim = min((double) p, i + val);
      mi = min(mi, make_pair(abs(p - lim), lim));
    }
  }
  return mi.second + perim;
}

int main(void) {
  int t;
  cin >> t;
  REP(case_nr, 1, t + 1) {
    int n;
    int p;
    scanf("%d%d",&n,&p);
    VI w(n), h(n);
    REP(i, 0, n) scanf("%d%d",&w[i],&h[i]);
    double ans = solve(n, p, w, h);
    printf("Case #%d: %.15f\n", case_nr, ans);
  }
}
