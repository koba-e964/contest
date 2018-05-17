#include <iostream>
#include <string>
#include <algorithm>
#include <queue>
#include <utility>
#include <vector>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<int, ll> PIL;
typedef pair<ll, PI> PLPI;

const int N = 100100;
vector<PLPI> g[N];

int main(void) {
  int n, m, k;
  cin >> n >> m >> k;
  REP(i, 0, m) {
    int u, v;
    ll x;
    cin >> u >> v >> x;
    u--, v--;
    g[u].push_back(PLPI(x, PI(v, -1)));
    g[v].push_back(PLPI(x, PI(u, -1)));
  }
  REP(i, 0, k) {
    int s;
    ll y;
    cin >> s >> y;
    s--;
    g[0].push_back(PLPI(y, PI(s, i)));
  }
  VL dist(n, 1e18);
  priority_queue<PLPI, vector<PLPI>, greater<PLPI> > que;
  que.push(PLPI(0, PI(0, -1)));
  VI nec(k);
  while (not que.empty()) {
    PLPI top = que.top(); que.pop();
    ll d = top.first;
    PI vs = top.second;
    int v = vs.first;
    int src = vs.second;
    if (dist[v] <= d) continue;
    dist[v] = d;
    if (src >= 0) {
      nec[src] = 1;
    }
    REP(j, 0, g[v].size()) {
      PLPI cws = g[v][j];
      ll c = cws.first;
      PI ws = cws.second;
      if (dist[ws.first] <= d + c) continue;
      que.push(PLPI(d + c, ws));
    }
  }
  int tot = 0;
  REP(i, 0, k) {
    tot += nec[i];
  }
  cout << k - tot << endl;
}
