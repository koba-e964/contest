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

const int N = 123456;
VI edges[N];
int n, m;

bool vis[N];
int col[N];

PI dfs(int v, int c = 0) {
  if (vis[v]) {
    if (c == col[v]) {
      return PI(0, 0);
    } else {
      return PI(0, -1);
    }
  }
  vis[v] = true;
  col[v] = c;
  int t0 = 0;
  int t1 = 0;
  bool bip = true;
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    PI res = dfs(w, 1 - c);
    if (res.second == -1) {
      bip = false;
      t0 += res.first;
    } else {
      t0 += res.first;
      t1 += res.second;
    }
  }
  if (c == 0) {
    t0 += 1;
  } else {
    t1 += 1;
  }
  return bip ? PI(t0, t1) : PI(t0 + t1, -1);
}

int main(void) {
  cin >> n >> m;
  REP(i, 0, m) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    edges[a].push_back(b);
    edges[b].push_back(a);
  }
  ll tot = 0;
  REP(i, 0, n) {
    if (not vis[i]) {
      PI res = dfs(i);
      ll nc = res.first;
      if (res.second == -1) { // not bipartite
	tot += nc * (nc - 1) / 2;
      } else {
	ll nd = res.second;
	tot += nc * nd;
      }
    }
  }
  cout << tot - m << endl;
}
