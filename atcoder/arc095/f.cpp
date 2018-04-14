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

/**
 * Manages a tree and calculates the diameter of it.
 * Header requirement: algorithm, vector
 * Verified by: NJPC 2017-E
 *              (http://njpc2017.contest.atcoder.jp/submissions/1089479)
 */
class Diameter {
  int n;
  typedef long long ll;
  typedef std::pair<int, ll> pil;
  std::vector<std::vector<pil> > edges;
  int x, y;
  void dfs(int v, std::vector<ll> &dist, int p = -1, ll d = 0) const {
    dist[v] = d;
    for (int i = 0; i < edges[v].size(); ++i) {
      int w = edges[v][i].first;
      if (w == p) { continue; }
      dfs(w, dist, v, d + edges[v][i].second);
    }
  }
public:
  Diameter(int n) : n(n), edges(n), x(-1), y(-1) {}
  void add_edge(int a, int b, ll c) {
    edges[a].push_back(pil(b, c));
    edges[b].push_back(pil(a, c));
  }
  std::pair<int, int> diameter(void) {
    if (x >= 0) {
      return std::pair<int, int>(x, y);
    }
    std::vector<ll> dist(n);
    dfs(0, dist);
    int maxi = 0;
    for (int i = 1; i < n; ++i) {
      if (dist[maxi] < dist[i]) {
	maxi = i;
      }
    }
    x = maxi;
    dfs(maxi, dist);
    for (int i = 0; i < n; ++i) {
      if (dist[maxi] < dist[i]) {
	maxi = i;
      }
    }
    y = maxi;
    return std::pair<int, int>(x, y);
  }
  VL dist(int x) {
    std::vector<ll> dist(n);
    dfs(x, dist);
    return dist;
  }
  std::vector<ll> farthest(void) {
    if (x < 0) {
      diameter();
    }
    std::vector<ll> ret(n, 0), tmp(n);
    /* For every vertex, the farthest point from it is either x or y. */
    dfs(x, tmp);
    for (int i = 0; i < n; ++i) {
      ret[i] = tmp[i];
    }
    dfs(y, tmp);
    for (int i = 0; i < n; ++i) {
      ret[i] = std::max(ret[i], tmp[i]);
    }
    return ret;
  }
};


const int N = 100100;
VI edges[N];


VI get(int x, int y, Diameter &diam, int n) {
  VL dis = diam.dist(x);
  int rem = dis[y];
  int cur = y;
  int pre = -1;
  VI cons;
  while (rem > 0) {
    if (DEBUG) {
      cerr << "rem = " << rem << " cur = " << cur << endl;
    }
    int nxt = -1;
    REP(i, 0, edges[cur].size()) {
      int w = edges[cur][i];
      if (dis[w] == rem - 1) {
	nxt = w;
	break;
      }
    }
    REP(i, 0, edges[cur].size()) {
      int w = edges[cur][i];
      if (w == nxt || w == pre) continue;
      if (edges[w].size() != 1) {
	return VI();
      }
    }
    if (y != cur) {
      cons.push_back(edges[cur].size() - (y == cur ? 0 : 1));
    }
    pre = cur;
    cur = nxt;
    rem -= 1;
  }
  cons.push_back(1);
  reverse(cons.begin(), cons.end());
  if (DEBUG) {
    cerr << "cons:";
    REP(i, 0, cons.size()) cerr << " " << cons[i];
    cerr << endl;
  }
  // TODO
  VI p(n);
  int tot = 0;
  REP(i, 0, cons.size()) {
    int v = cons[i];
    p[tot + v - 1] = tot + 1;
    REP(j, 1, v) {
      p[tot + j - 1] = tot + j + 1;
    }
    tot += v;
  }
  p[n - 1] = n;
  if (DEBUG) {
    cerr << "p:";
    REP(i, 0, n) {
      cerr << " " << p[i];
    }
    cerr << endl;
  }
  return p;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  Diameter diam(n);
  REP(i, 0, n - 1) {
    int u, v;
    cin >> u >> v;
    u--, v--;
    edges[u].push_back(v);
    edges[v].push_back(u);
    diam.add_edge(v, u, 1);
  }
  PI di = diam.diameter();
  int x = di.first;
  int y = di.second;
  VI ans(1, 1e8);
  REP(_, 0, 2) {
    VI tmp = get(x, y, diam, n);
    if (tmp.size() > 0) {
      ans = min(ans, tmp);
    }
    swap(x, y);
  }
  if (ans[0] == 1e8) {
    cout << -1 << endl;
  } else {
    REP(i, 0, n) {
      cout << ans[i] << (i == n - 1 ? "\n" : " ");
    }
  }
}
