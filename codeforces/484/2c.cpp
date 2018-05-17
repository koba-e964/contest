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
int ch[N];

void dfs(int v, int par) {
  int tot = 1;
  REP(i, 0, g[v].size()) {
    int w = g[v][i];
    if (par == w) continue;
    dfs(w, v);
    tot += ch[w];
  }
  ch[v] = tot;
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
  if (n % 2 == 1) {
    cout << -1 << endl;
    return 0;
  }
  dfs(0, -1);
  int tot = 0;
  REP(i, 1, n) {
    tot += ch[i] % 2 == 0 ? 1 : 0;
  }
  cout << tot << endl;
}
