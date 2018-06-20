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

const int N = 100100;
VI g[N];
bool vis[N];
int col[N];

bool is_bipartite(int v, int c) {
  if (vis[v]) {
    return c == col[v];
  }
  vis[v] = true;
  col[v] = c;
  for (int w: g[v]) {
    if (not is_bipartite(w, 1 - c)) {
      return false;
    }
  }
  return true;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 0, n) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    g[a].push_back(b);
    g[b].push_back(a);
  }
  int mi = n + 1;
  int mincut, maxcut = -1;
  REP(i, 0, n) {
    mi = min(mi, (int) g[i].size());
  }
  mincut = mi;
  maxcut = n - (is_bipartite(0, 0) ? 0 : 1);
  cout << mincut << " " << maxcut << endl;
}
