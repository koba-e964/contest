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
const ll mod = 1e9 + 7;

void fail(void) {
  cout << "IMPOSSIBLE" << endl;
  exit(0);
}

const int N = 1192;
int p[N], x[N];
VI child[N];
int opp[N];


int lux(int tot, const vector<PI> &dat) {
  int n = dat.size();
  vector<vector<bool> > dp(n + 1, vector<bool>(tot + 1, false));
  dp[0][0] = true;
  REP(i, 0, n) {
    int u = dat[i].first;
    int v = dat[i].second;
    REP(j, 0, tot + 1) {
      if (j >= u) {
	dp[i + 1][j] = dp[i + 1][j] || dp[i][j - u];
      }
      if (j >= v) {
	dp[i + 1][j] = dp[i + 1][j] || dp[i][j - v];
      }
    }
  }
  int sol = -1;
  REP(i, 0, tot + 1) {
    if (dp[n][i]) {
      sol = i;
    }
  }
  return sol;
}


void dfs(int v) {
  vector<PI> dat;
  int tot = 0;
  REP(i, 0, child[v].size()) {
    int w = child[v][i];
    dfs(w);
    dat.push_back(PI(x[w], opp[w]));
    tot += x[w] + opp[w];
  }
  int res = lux(x[v], dat);
  if (res < 0) {
    fail();
  }
  opp[v] = tot - res;
}


int main(void) {
  int n;
  cin >> n;
  p[0] = -1;
  REP(i, 1, n) {
    cin >> p[i];
    p[i]--;
    child[p[i]].push_back(i);
  }
  REP(i, 0, n) {
    cin >> x[i];
  }
  /*
  REP(i, 0, n) {
    // check a -> b -> i
    int b = p[i];
    if (b == -1) { continue; }
    int a = p[b];
    if (a == -1) { continue; }
    if (x[a] < x[b] && x[b] < x[i]) {
      fail();
    }
  }
  */
  dfs(0);
  cout << "POSSIBLE" << endl;
}
