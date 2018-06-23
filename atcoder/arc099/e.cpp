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

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 701;
VI g[N];
int sigma[N][N];

int col[N];
bool vis[N];
PI dfs(int v, int c) {
  if (vis[v]) {
    if (col[v] != c) {
      return PI(-1, -1);
    }
    return PI(0, 0);
  }
  vis[v] = 1;
  col[v] = c;
  int tap = 1, ris = 0;
  for (int w: g[v]) {
    PI sub = dfs(w, 1 - c);
    if (sub.first == -1) return sub;
    tap += sub.second;
    ris += sub.first;
  }
  return PI(tap, ris);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  REP(i, 0, m) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    sigma[a][b] = 1;
    sigma[b][a] = 1;
  }
  REP(i, 0, n) REP(j, 0, n) {
    if (i != j) sigma[i][j] = 1 - sigma[i][j];
  }
  // complement
  REP(i, 0, n) {
    REP(j, 0, n) {
      if (sigma[i][j]) {
        g[i].push_back(j);
      }
    }
  }
  vector<PI> yuki;
  REP(i, 0, n) {
    if (not vis[i]) {
      PI res = dfs(i, 0);
      if (res.first == -1) {
        cout << -1 << endl;
        return 0;
      }
      yuki.push_back(res);
    }
  }
  if (DEBUG) {
    for (PI y: yuki) cerr << " " << y.first << "," << y.second;
    cerr << endl;
  }
  typedef bitset<N> bs;
  bs resha;
  resha[0] = 1;
  for (PI y: yuki) {
    bs tmp = resha << y.first;
    tmp |= resha << y.second;
    resha = tmp;
  }
  int ma = 1e8;
  REP(i, 0, N) {
    if (resha[i]) {
      ma = min(ma, i * (i - 1) / 2 + (n - i) * (n - i - 1) / 2);
    }
  }
  cout << ma << endl;
}
