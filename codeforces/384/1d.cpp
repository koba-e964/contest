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
const ll mod = 1e9 + 7;

const int N = 200100;
VI edges[N];
ll a[N];
ll b[N];
ll c[N];
void dfs1(int v, int par) {
  ll sum = a[v];
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    if (w == par) { continue; }
    dfs1(w, v);
    sum += b[w];
  }
  b[v] = sum;
}

void dfs2(int v, int par) {
  ll mi = b[v];
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    if (w == par) { continue; }
    dfs2(w, v);
    mi = max(mi, c[w]);
  }
  c[v] = mi;
}

void dfs3(int v, int par, ll &ma) {
  VL vals;
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    if (w == par) { continue; }
    dfs3(w, v, ma);
    vals.push_back(c[w]);
  }
  sort(vals.rbegin(), vals.rend());
  if (vals.size() >= 2) {
    ma = max(ma, vals[0] + vals[1]);
  }
}

ll minf = -1e16;

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> a[i];
  }
  REP(i, 0, n - 1) {
    int u, v;
    cin >> u >> v;
    u--, v--;
    edges[u].push_back(v);
    edges[v].push_back(u);
  }
  dfs1(0, -1);
  dfs2(0, -1);
  ll ma = minf;
  dfs3(0, -1, ma);
  if (ma == minf) {
    cout << "Impossible" << endl;
  } else {
    cout << ma << endl;
  }
}
