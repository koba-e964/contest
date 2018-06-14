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
int diff[N];
int hot[N];

void dfs(int v, int par, VI hh) {
  if (diff[v] ^ hh[0]) {
    hot[v] = 1;
    hh[0] ^= 1;
  }
  swap(hh[0], hh[1]);
  for (int w: g[v]) {
    if (par == w) continue;
    dfs(w, v, hh);
  }
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 0, n - 1) {
    int u, v;
    cin >> u >> v;
    u--, v--;
    g[u].push_back(v);
    g[v].push_back(u);
  }
  REP(i, 0, n) cin >> diff[i];
  REP(i, 0, n) {
    int y;
    cin >> y;
    diff[i] ^= y;
  }
  dfs(0, -1, VI(2, 0));
  int ans = 0;
  REP(i, 0, n) ans += hot[i];
  cout << ans << "\n";
  REP(i, 0, n) if (hot[i]) cout << i + 1 << "\n";
}
