#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;

const int N = 100100;
int rr[N], cc[N];
ll a[N];

vector<PL> cons[N];

int r, c, n;
vector<PL> edges[N];
ll dp[N];
int root[N];
void update_cons(int c1, int c2, ll d) {
  edges[c1].push_back(PL(c2, d));
  edges[c2].push_back(PL(c1, -d));
}

ll inf = -1e16;
bool dfs(int v, ll c, int r) {
  root[v] = r;
  if (dp[v] > inf) {
    return dp[v] == c;
  }
  dp[v] = c;
  REP(i, 0, edges[v].size()) {
    PL e = edges[v][i];
    bool res = dfs(e.first, c + e.second, r);
    if (!res) {
      return false;
    }
  }
  return true;
}

int main(void){
  cin >> r >> c >> n;
  REP(i, 0, n) {
    cin >> rr[i] >> cc[i] >> a[i];
    rr[i]--, cc[i]--;
    cons[rr[i]].push_back(PL(cc[i], a[i]));
  }
  REP(i, 0, r) {
    sort(cons[i].begin(), cons[i].end());
    if (cons[i].size() <= 1) {
      continue; // no constraints
    }
    REP(j, 0, cons[i].size() - 1) {
      update_cons(cons[i][j].first, cons[i][j + 1].first,
			     cons[i][j + 1].second - cons[i][j].second);
    }
  }
  REP(i, 0, c) {
    dp[i] = inf;
  }
  REP(i, 0, c) {
    bool res = 1;
    if (dp[i] <= inf) {
      res = dfs(i, 0, i);
    }
    if (!res) {
      cout << "No" << endl; // contradiction!!
      return 0;
    }
  }
  // no obvious contradiction found.
  VL mi(c, 0);
  REP(i, 0, c) {
    if (dp[i] > inf) {
      mi[root[i]] = min(mi[root[i]], dp[i]);
    }
  }
  /*
  REP(i, 0, c) {
    cerr << i << " dp " << dp[i] << " root " << root[i] << endl;
    if (root[i] == i) {
      cerr << "min[" << i << "]: " << mi[i] << endl;
    }
  }
  */
  bool ok = 1;
  REP(i, 0, n) {
    if (dp[cc[i]] > inf && a[i] - dp[cc[i]] + mi[root[cc[i]]] < 0) {
      ok = 0; // row rr[i] went wrong
    }
  }
  cout << (ok ? "Yes" : "No") << endl;
}
