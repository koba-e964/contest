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


/**
 * Dijkstra's algorithm.
 * First, call add_edge() to add edges.
 * Second, call solve() to calculate the length of the shortest path from source to each vertex.
 * Header requirement: algorithm, queue, vector
 * Verified by AtCoder ARC026-C (http://arc026.contest.atcoder.jp/submissions/604231)
 */
 template<class Len = int>
class Dijkstra {
private:
  int n;
  std::vector<std::vector<std::pair<int, Len> > > edges;
public:
  /**
   * n: the number of vertices
   */
  Dijkstra(int n) : n(n), edges(n) {}
  /*
   * from: the source of edge to add
   * to: the target of edge to add
   * cost: the cost of edge to add
   */
  void add_edge(int from, int to, Len cost) {
    edges[from].push_back(std::pair<int, Len>(to, cost));
  }
  /*
   * This function returns an array consisting of the distances from vertex source.
   */
  std::vector<Len> solve(int source) {
    const Len inf = 1e16;
    typedef std::pair<Len, int> pi;
    std::vector<Len> d(n, inf);
    std::priority_queue<pi, std::vector<pi>, std::greater<pi> > que;
    que.push(pi(0, source));
    while (!que.empty()) {
      pi p = que.top(); que.pop();
      int idx = p.second;
      if (d[idx] <= p.first) {
	continue;
      }
      d[idx] = p.first;
      for(int j = 0; j < edges[idx].size(); ++j) {
	que.push(pi(p.first + edges[idx][j].second, edges[idx][j].first));
      }
    }
    return d;
  }
};

const int DEBUG = 0;

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<int, ll> PIL;
typedef pair<ll, int> PLI;
const ll mod = 1e9 + 7;

ll sq(ll x) { return x * x % mod; }

const int N = 100100;
int n;
vector<PIL> edges[N];
bool sh[N];

void calc(int s, VL &dp, const VL &sol_s, const VL &sol_t) {
  vector<bool> vis(n, false);
  priority_queue<PLI, vector<PLI> , greater<PLI> > que;
  dp[s] = 1;
  assert (sol_s[s] == 0);
  que.push(PLI(0, s));
  while (not que.empty()) {
    PLI vd = que.top(); que.pop();
    int v = vd.second;
    if (vis[v]) continue;
    vis[v] = true;
    REP(i, 0, edges[v].size()) {
      PIL wc = edges[v][i];
      int w = wc.first;
      ll c = wc.second;
      if (not sh[w])continue;
      if (sol_s[v] + c != sol_s[w]) continue;
      que.push(PLI(sol_s[w], w));
      dp[w] += dp[v];
      dp[w] %= mod;
    }
  }
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int m, s, t;
  cin >> n >> m >> s >> t;
  s--, t--;
  Dijkstra<ll> dijk(n);
  REP(i, 0, m) {
    int u, v, d;
    cin >> u >> v >> d;
    u--, v--;
    dijk.add_edge(u, v, d);
    dijk.add_edge(v, u, d);
    edges[u].push_back(PIL(v, d));
    edges[v].push_back(PIL(u, d));
  }
  VL sol_s, sol_t;
  sol_s = dijk.solve(s);
  sol_t = dijk.solve(t);
  REP(i, 0, n) {
    if (sol_s[i] + sol_t[i] == sol_s[t]) {
      if (DEBUG) DEBUGP(i);
      sh[i] = true;
    }
  }
  VL dp(n), dp2(n);
  calc(s, dp, sol_s, sol_t);
  calc(t, dp2, sol_t, sol_s);
  if (DEBUG) {
    cerr << "dp:";
    REP(i, 0, n) cerr << " " << dp[i];
    cerr << endl;
    cerr << "dp2:";
    REP(i, 0, n) cerr << " " << dp2[i];
    cerr << endl;
  }
  assert (dp[t] == dp2[s]);
  REP(i, 0, n) {
    assert (dp[i] < mod);
    assert (dp[i] >= 0);
    assert (dp2[i] < mod);
    assert (dp2[i] >= 0);
  }
  ll ans = sq(dp[t]);
  REP(i, 0, n) {
    if (not sh[i]) continue;
    if (2 * sol_s[i] == sol_s[t]) {
      ans += mod - sq(dp[i] * dp2[i] % mod);
      ans %= mod;
    }
    if (2 * sol_s[i] < sol_s[t]) {
      REP(j, 0, edges[i].size()) {
	PIL wc = edges[i][j];
	int w = wc.first;
	ll c = wc.second;
	if (not sh[w])continue;
	if (sol_s[i] + c != sol_s[w]) continue;
	if (2 * sol_s[w] > sol_s[t]) {
	  ans += mod - (sq(dp[i] * dp2[w] % mod) % mod);
	  ans %= mod;
	}
      }
    }
  }
  cout << ans << endl;
}
