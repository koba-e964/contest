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

const int N = 210000;
VI g[N];
string s;

bool vis[N];
int possum[N]; // 2: indeterminate
bool dfs(int v) {
  if (vis[v]) return possum[v] != 0;
  int a = 0;
  int b = 0;
  vis[v] = true;
  possum[v] = 2;
  for (int w: g[v]) {
    if (dfs(w)) {
      if (s[w] == 'A') a = 1;
      else b = 1;
    }
    if (a == 1 && b == 1) {
      possum[v] = 1;
      return true;
    }
  }
  possum[v] = 0;
  return false;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m >> s;
  REP(i, 0, m) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    g[a].push_back(b);
    g[b].push_back(a);
  }
  VI ac(n), bc(n); // count of adjacent vertices
  REP(i, 0, n) {
    for (int w: g[i]) {
      if (s[w] == 'A') ac[i]++;
      else bc[i]++;
    }
  }
  queue<int> que;
  vector<bool> alive(n, true);
  REP(i, 0, n) {
    if (ac[i] == 0 || bc[i] == 0) que.push(i);
  }
  while (not que.empty()) {
    int v = que.front(); que.pop();
    if (not alive[v]) continue;
    // eliminate v
    alive[v] = false;
    for (int w: g[v]) {
      if (s[v] == 'A') ac[w]--;
      else bc[w]--;
      if (ac[w] == 0 || bc[w] == 0) {
	que.push(w);
      }
    }
  }
  bool ok = false;
  REP(i, 0, n) ok |= alive[i];
  cout << (ok ? "Yes" : "No") << endl;
}
