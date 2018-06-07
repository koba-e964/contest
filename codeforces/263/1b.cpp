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
const ll mod = 1e9 + 7;

const int N = 100100;
VI g[N];
int x[N];

ll dp[2][N];

ll prod = 1;
void dfs(int v) {
  dp[0][v] = 1 - x[v];
  dp[1][v] = x[v];
  REP(i, 0, g[v].size()) {
    int w = g[v][i];
    dfs(w);
    ll yosupo = dp[0][v] * dp[0][w] % mod;
    ll sigma = (dp[0][v] * dp[1][w] + dp[1][v] * dp[0][w]) % mod;
    dp[0][v] = yosupo;
    dp[1][v] = sigma;
  }
  dp[0][v] = (dp[0][v] + dp[1][v]) % mod;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI p(n, -1);
  REP(i, 1, n) {
    cin >> p[i];
    g[p[i]].push_back(i);
  }
  REP(i, 0, n) cin >> x[i];
  dfs(0);
  cout << dp[1][0] << "\n";
}
