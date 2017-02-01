#include <vector>
#include <algorithm>
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
#include <algorithm>
#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<ll> VL;
typedef pair<int, int> PI;


const int N = 100100;
vector<PI> edges[N];
int dp[N];

// Counts #flips needed at Vertex v in the subtree whose root is Vertex v
int dfs(int v, int p = -1) {
  int tot = 0;
  for (auto w: edges[v]) {
    if (w.first == p) { continue; }
    tot += dfs(w.first, v) + w.second;
  }
  return tot;
}

// Couns #flips needed at Vertex v, with the aid of fc.
void dfs2(int v, int fc, int p = -1) {
  dp[v] = fc;
  for (auto w: edges[v]) {
    if (w.first == p) { continue; }
    dfs2(w.first, fc + (w.second ? -1 : 1), v);
  }
}

// This solution was made after the author read the editorial.
int main(void){
  int n;
  ll d;
  cin >> n >> d;
  Diameter dia(n);
  REP(i, 0, n - 1) {
    int a, b, c;
    cin >> a >> b >> c;
    a--, b--;
    edges[a].push_back(PI(b, 0));
    edges[b].push_back(PI(a, 1));
    dia.add_edge(a, b, c);
  }
  VL farth = dia.farthest();
  // check directions
  int flip0 = dfs(0);
  dfs2(0, flip0);
  
  int mi = n;
  REP(i, 0, n) {
    if (farth[i] <= d) {
      mi = min(mi, n - 1 - dp[i]);
    }
  }
  cout << (mi == n ? -1 : mi) << endl;
}
