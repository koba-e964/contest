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
const ll mod = 1e9 + 7;

const int N = 123456;

VI edges[N];
double dp[N];

void dfs(int v, int par) {
  int nc = 0;
  double sum = 0.0;
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    if (w == par) { continue; }
    nc += 1;
    dfs(w, v);
    sum += dp[w];
  }
  dp[v] = nc >= 1 ? 1.0 + sum / nc : 0.0;
}


int main(void) {
  int n;
  scanf("%d", &n);
  REP(i, 0, n - 1) {
    int u, v;
    scanf("%d%d", &u, &v);
    u--, v--;
    edges[u].push_back(v);
    edges[v].push_back(u);
  }
  dfs(0, -1);
  printf("%.15f\n", dp[0]);
}
