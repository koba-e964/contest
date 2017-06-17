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

const int N = 5142;
ll c[N], d[N];
int x[N];
VI edges[N];
ll dp[N][N][2];
int sz[N];
const ll inf = 1e15;
ll nxt[N][2];

void dfs(int v) {
  sz[v] = 1;
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    dfs(w);
    sz[v] += sz[w];
  }
  
  // update dp[v][][]
  dp[v][0][0] = 0;
  dp[v][0][1] = inf;
  dp[v][1][0] = c[v];
  dp[v][1][1] = c[v] - d[v];
  int cnt = 1;
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    REP(j, 0, cnt + sz[w] + 1) {
      REP(b, 0, 2) {
	nxt[j][b] = inf;
      }
    }
    REP(j, 0, cnt + 1) {
      REP(k, 0, sz[w] + 1) {
	nxt[j + k][0] = min(nxt[j + k][0], dp[v][j][0] + dp[w][k][0]);
	nxt[j + k][1] = min(nxt[j + k][1], dp[v][j][1] + dp[w][k][0]);
	nxt[j + k][1] = min(nxt[j + k][1], dp[v][j][1] + dp[w][k][1]);
      }
    }
    cnt += sz[w];
    REP(j, 0, cnt + 1) {
      REP(b, 0, 2) {
	dp[v][j][b] = nxt[j][b];
      }
    }
  }
  assert (cnt == sz[v]);
}

// Reference: http://codeforces.com/contest/815/submission/27853891
// by yutaka1999
int main(void){
  cin.tie(0);
  ios::sync_with_stdio(false);
  int n;
  ll b;
  cin >> n >> b;
  REP(i, 0, n) {
    cin >> c[i] >> d[i];
    if (i >= 1) {
      cin >> x[i];
      x[i]--;
      edges[x[i]].push_back(i);
    } else {
      x[i] = -1;
    }
  }
  dfs(0);
  int ma = 0;
  REP(j, 0, sz[0] + 1) {
    if (dp[0][j][0] <= b || dp[0][j][1] <= b) {
      ma = max(ma, j);
    }
  }
  cout << ma << endl;
}
