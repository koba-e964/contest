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

const int N = 123456;

vector<PI> edges[N];


ll dist[N];


void dfs(int v, int p, ll tot) {
  dist[v] = tot;
  REP(i, 0, edges[v].size()) {
    PI wd = edges[v][i];
    int w = wd.first;
    if (w == p) continue;
    int d = wd.second;
    dfs(w, v, tot + d);
  }
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 0, n - 1) {
    int a, b, c;
    cin >> a >> b >> c;
    a--, b--;
    edges[a].push_back(PI(b, c));
    edges[b].push_back(PI(a, c));
  }
  int q, k;
  cin >> q >> k;
  k--;
  dfs(k, -1, 0);
  REP(i, 0, q) {
    int x, y;
    cin >> x >> y;
    x--, y--;
    cout << dist[x] + dist[y] << endl;
  }
}
