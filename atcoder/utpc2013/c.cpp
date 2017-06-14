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

const int N = 1010;

VI edges1[N], edges2[N];

const int inf = 1e7;

// max distance table
int dp1[N], dp2[N];

void calc_dist(int n, VI *edges, int dp[N]) {
  REP(i, 0, n) {
    VI dis(n, inf);
    queue<PI> que;
    que.push(PI(0, i));
    while (not que.empty()) {
      PI t = que.front(); que.pop();
      int v = t.second;
      int d = t.first;
      if (dis[v] <= d) { continue; }
      dis[v] = d;
      REP(j, 0, edges[v].size()) {
	int w = edges[v][j];
	if (dis[w] <= d + 1) { continue; }
	que.push(PI(d + 1, w));
      }
    }
    int ma = 0;
    REP(j, 0, n) {
      ma = max(ma, dis[j]);
    }
    dp[i] = ma;
  }
}

int main(void){
  int n1, m1;
  cin >> n1 >> m1;
  REP(i, 0, m1) {
    int u, v;
    cin >> u >> v;
    edges1[u].push_back(v);
    edges1[v].push_back(u);
  }
  int n2, m2;
  cin >> n2 >> m2;
  REP(i, 0, m2) {
    int u, v;
    cin >> u >> v;
    edges2[u].push_back(v);
    edges2[v].push_back(u);
  }
  calc_dist(n1, edges1, dp1);
  calc_dist(n2, edges2, dp2);
  int dia1 = 0;
  int dia2 = 0;
  REP(i, 0, n1) {
    dia1 = max(dia1, dp1[i]);
  }
  REP(i, 0, n2) {
    dia2 = max(dia2, dp2[i]);
  }
  int ma = 0;
  int mi = inf;
  REP(i, 0, n1) {
    REP(j, 0, n2) {
      ma = max(ma, dp1[i] + 1 + dp2[j]);
      mi = min(mi, max(dp1[i] + 1 + dp2[j], max(dia1, dia2)));
    }
  }
  cout << mi << " " << ma << endl;
}
