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

const int N = 300300;

VI edges[N];
vector<PI> cycles;
int ord[N];
int pre[N];
int cnt = 0;

void dfs(int v, int par) {
  if (ord[v] >= 0) {
    // cycle detected
    VI cycle;
    cycle.push_back(v);
    int cur = par;
    while (cur != v) {
      cycle.push_back(cur);
      cur = pre[cur];
    }
    sort(cycle.begin(), cycle.end());
    assert (cycle.size() >= 3);
    cycles.push_back(PI(cycle[0], cycle[cycle.size() - 1]));
    return;
  }
  ord[v] = cnt;
  pre[v] = par;
  cnt += 1;
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    if (par == w) { continue; }
    if (ord[w] == -1 || ord[w] < ord[v]) {
      dfs(w, v);
    }
  }
}



// This solution was written after the author read the editorial
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  REP(i, 0, m) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    edges[a].push_back(b);
    edges[b].push_back(a);
  }
  REP(i, 0, n) {
    ord[i] = -1;
  }
  REP(i, 0, n) {
    if (ord[i] < 0) {
      dfs(i, -1);
    }
  }
  VI nxt(n, n - 1);
  REP(i, 0, cycles.size()) {
    int x = cycles[i].first;
    int y = cycles[i].second;
    nxt[x] = min(nxt[x], y - 1);
  }
  for (int i = n - 2; i >= 0; --i) {
    nxt[i] = min(nxt[i], nxt[i + 1]);
  }
  VL acc(n + 1, 0);
  REP(i, 0, n) {
    acc[i + 1] = acc[i] + nxt[i];
  }
  int q;
  cin >> q;
  REP(i, 0, q) {
    int l, r;
    cin >> l >> r;
    l--, r--;
    int pass = r;
    int fail = l - 1;
    while (pass - fail > 1) {
      int mid = (pass + fail) / 2;
      if (nxt[mid] >= r) {
	pass = mid;
      } else {
	fail = mid;
      }
    }
    ll tot = acc[pass] - acc[l];
    tot += (ll)r * (r - pass + 1);
    tot -= (ll) (l + r - 2) * (r - l + 1) / 2;
    cout << tot << "\n";
  }
}
