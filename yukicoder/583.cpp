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

const int N = 512;
VI edges[N];

bool vis[N];

void dfs(int v) {
  if (vis[v]) {
    return;
  }
  vis[v] = true;
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    dfs(w);
  }
}

void fail(void) {
  cout << "NO\n";
  exit(0);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  VI deg(n);
  int start = -1;
  REP(i, 0, m) {
    int u, v;
    cin >> u >> v;
    edges[u].push_back(v);
    edges[v].push_back(u);
    deg[u] += 1;
    deg[v] += 1;
    if (start == -1) {
      start = u;
    }
  }
  // Check degree
  int odd = 0;
  REP(i, 0, n) {
    odd += deg[i] % 2;
  }
  if (odd >= 4) {
    fail();
  }
  dfs(start);
  REP(i, 0, n) {
    if (deg[i] > 0 && not vis[i]) {
      fail();
    }
  }
  cout << "YES\n";
}
