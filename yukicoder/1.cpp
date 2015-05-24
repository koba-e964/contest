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
typedef pair<int, int> PI;
const double EPS=1e-9;

const int inf = 123456789;
int n, c, v;
const int V = 1510;
const int N = 54;

int s[V], t[V], y[V], m[V];

struct edge {
  int to, cost, dist;
};

const int C = 310;

int dp[N * C]; // 2-dim

const int DEBUG = 0;
vector<edge> adj[N];

int main(void){
  cin >> n >> c >> v;
  REP(i, 0, v) {
    cin >> s[i];
  }
  REP(i, 0, v) {
    cin >> t[i];
  }
  REP(i, 0, v) {
    cin >> y[i];
  }
  REP(i, 0, v) {
    cin >> m[i];
  }
  REP(i, 0, v) {
    s[i] --;
    t[i] --;
    adj[s[i]].push_back((edge) {t[i], y[i], m[i]});
  }
  REP(i, 0, n) {
    REP(j, 0, c + 1) {
      dp[i * C + j] = inf;
    }
  }
  priority_queue<PI, vector<PI>, greater<PI> > que; // queue of (place, cost)
  que.push(PI(0, 0));
  dp[0] = 0;
  while (! que.empty()) {
    PI p = que.top(); que.pop();
    int vt = p.second;
    if(dp[vt] < p.first) {
      continue;
    }
    int vn = vt / C;
    REP(i, 0, adj[vn].size()) {
      edge e = adj[vn][i];
      int totalcost = (vt % C) + e.cost;
      if (totalcost > c) {
	continue;
      }
      int to = e.to * C + totalcost;
      if (dp[to] > dp[vt] + e.dist) {
	if (DEBUG) {
	  cout << "update: e.to =" << e.to << ", totalcost = " << totalcost << endl;
	}
	dp[to] = dp[vt] +e.dist;
	que.push(PI(dp[to], to));
      }
    }
  }
  int m = inf;
  REP(i, 0, c + 1) {
    m = min(m, dp[(n - 1) * C + i]);
  }
  cout << (m == inf ? -1 : m) << endl;
}
