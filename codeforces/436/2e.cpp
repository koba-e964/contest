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

const int N = 111;
const int W = 2437;
int dp[N][W];
int pre[N][W];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  vector<pair<PI, PI> > dat;
  REP(i, 0, n) {
    int t, d, p;
    cin >> t >> d >> p;
    dat.push_back(make_pair(PI(d, i), PI(t, p)));
  }
  sort(dat.begin(), dat.end()); // sort by d
  REP(i, 0, n + 1) {
    REP(j, 0, W) {
      dp[i][j] = -1;
    }
  }
  dp[0][0] = 0;
  REP(i, 0, n) {
    int t, d, p;
    d = dat[i].first.first;
    t = dat[i].second.first;
    p = dat[i].second.second;
    REP(j, 0, W) {
      PI ma(-1, -1);
      if (t <= j && j < d) {
	if (dp[i][j - t] >= 0) {
	  ma = max(ma, PI(dp[i][j - t] + p, j - t));
	}
      }
      if (dp[i][j] >= 0) {
	ma = max(ma, PI(dp[i][j], j));
      }
      if (ma.first >= 0) {
	dp[i + 1][j] = ma.first;
	pre[i + 1][j] = ma.second;
      }
    }
  }
  if (0) {
    cerr << "dp:";
    REP(i, 0, n + 1) {
      cerr << i << ":";
      REP(j, 0, 20) {
	cerr << " " << dp[i][j];
      }
      cerr << endl;
    }
    cerr << endl;
  }
  PI ma(-1, -1);
  REP(j, 0, W) {
    ma = max(ma, PI(dp[n][j], j));
  }
  cout << ma.first << "\n";
  int cur = ma.second;
  VI path;
  for (int i = n - 1; i >= 0; --i) {
    int idx = dat[i].first.second;
    int newcur = pre[i + 1][cur];
    if (newcur != cur) {
      path.push_back(idx + 1);
    }
    cur = newcur;
  }
  reverse(path.begin(), path.end());
  cout << path.size() << "\n";
  REP(i, 0, path.size()) {
    cout << path[i] << (i == (int) path.size() - 1 ? "\n" : " ");
  }
}
