#include <algorithm>
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
const ll mod = 1e9 + 7;

void add(ll &x, ll y) {
  x = (x + y) % mod;
}

const int N = 1010;
const int K = 52;

VI edges[N];
ll dp[N][3][K];
ll dp2[3][K];

int n, k;

void dfs(int v, int par) {
  dp[v][0][0] = 1;
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    if (w == par) continue;
    dfs(w, v);
    memset(dp2, 0, sizeof(dp2));
    REP(j, 0, 3) {
      REP(l, 0, 3) {
	REP(x, 0, K) {
	  REP(y, 0, K - x) {
	    add(dp2[j][x + y], dp[v][j][x] * dp[w][l][y]);
	    int conn = x + y + (j == 0 && l == 0 ? 1 : 0);
	    if (j == 1 && l == 1) conn--;
	    if (j < 2 && l < 2 && conn < K && conn >= 0) {
	      add(dp2[j + 1][conn], dp[v][j][x] * dp[w][l][y]);
	    }
	  }
	}
      }
    }
    swap(dp[v], dp2);
  }
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n >> k;
  REP(i, 0, n - 1) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    edges[a].push_back(b);
    edges[b].push_back(a);
  }
  dfs(0, -1);
  ll tot = 0;
  REP(i, 0, 3) {
    add(tot, dp[0][i][k]);
  }
  cout << tot << endl;
}
