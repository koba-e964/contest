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

const int N = 200100;
int n, m, k;
set<int> g[N];
queue<int> rem;
int alive_cnt;
bool alive[N];
void del_edge(int v, int w) {
  if (not alive[v] || not alive[w]) return;
  assert (v != w);
  assert (g[w].count(v));
  g[w].erase(v);
  if ((int) g[w].size() == k - 1) {
    rem.push(w);
  }
  assert (g[v].count(w));
  g[v].erase(w);
  if ((int) g[v].size() == k - 1) {
    rem.push(v);
  }
}
void del(int v) {
  if (not alive[v]) return;
  for (int w: g[v]) {
    assert (g[w].count(v));
    g[w].erase(v);
    if ((int) g[w].size() == k - 1) {
      rem.push(w);
    }
  }
  g[v].clear();
  alive[v] = false;
  alive_cnt--;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n >> m >> k;
  VI x(m), y(m);
  REP(i, 0, m) {
    cin >> x[i] >> y[i];
    x[i]--, y[i]--;
    g[x[i]].insert(y[i]);
    g[y[i]].insert(x[i]);
  }
  alive_cnt = n;
  REP(i, 0, n) alive[i] = true;
  REP(i, 0, n) if ((int) g[i].size() < k) rem.push(i);
  VI ans(m);
  for (int i = m - 1; i >= 0; --i) {
    while (not rem.empty()) {
      int v = rem.front(); rem.pop();
      del(v);
    }
    ans[i] = alive_cnt;
    del_edge(x[i], y[i]);
  }
  REP(i, 0, m) cout << ans[i] << "\n";
}
