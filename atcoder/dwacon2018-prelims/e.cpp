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

const int N = 2010;
VI edges[N];
VI edges2[N];
int n;

VI dist(int v) {
  VI ret(n, 1e8);
  queue<PI> que;
  que.push(PI(0, v));
  while (not que.empty()) {
    PI t = que.front(); que.pop();
    int w = t.second;
    int d = t.first;
    ret[w] = d;
    REP(i, 0, edges[w].size()) {
      int tar = edges[w][i];
      if (ret[tar] <= d) continue;
      que.push(PI(d + 1, tar));
    }
  }
  return ret;
}


int path_recover(const VI &dist, int v, int d) {
  int cur = v;
  int val = dist[v];
  while (val > d && val >= 0) {
    REP(i, 0, edges[cur].size()) {
      int w = edges[cur][i];
      if (dist[w] == val - 1) {
	cur = w;
	val -= 1;
	break;
      }
    }
  }
  return cur;
}

void cut(int v, int par, const set<int> &forbid) {
  if (DEBUG) {
    cerr << "cut" << "v = " << v << " " << par << endl;
  }
  VI cp = edges[v];
  VI res;
  REP(i, 0, cp.size()) {
    int w = cp[i];
    if (forbid.count(w)) continue;
    res.push_back(w);
    if (par == w) continue;
    cut(w, v, forbid);
  }
  edges2[v] = res;
  if (par == -1) {
    REP(i, 0, n) {
      edges[i] = edges2[i];
    }
  }
}


// http://techtipshoge.blogspot.jp/2016/09/centroid-decomposition.html
int sz[N];

void szdfs(int v, int par = -1) {
  sz[v] = 1;
  for (auto &w: edges[v]) {
    if (w == par) continue;
    szdfs(w, v);
    sz[v] += sz[w];
  }
}

int centroid(int v, int par, int total) {
  for (auto &w: edges[v]) {
    if (w == par) continue;
    if (2 * sz[w] > total)
      return centroid(w, v, total);
  }
  return v;
}
void get_diam(int &u, int &v, VI &du, VI &dv, int piv) {
  szdfs(piv, -1);
  int ctr = centroid(piv, -1, sz[piv]);
  int ma;
  szdfs(ctr);
  vector<PI> tbl;
  REP(i, 0, edges[ctr].size()) {
    int nxt = edges[ctr][i];
    tbl.push_back(PI(sz[nxt], nxt));
  }
  sort(tbl.rbegin(), tbl.rend());
  if (tbl.size() == 0) {
    u = v = ctr;
  } else if (tbl.size() == 1) {
    u = tbl[0].second;
    v = ctr;
  } else {
    u = tbl[0].second;
    v = tbl[1].second;
  }
  du = dist(u);
  dv = dist(v);
}

int main(void) {
  int q;
  cin >> n >> q;
  REP(i, 0, n - 1) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    edges[a].push_back(b);
    edges[b].push_back(a);
  }
  int u;
  VI du;
  int v;
  VI dv;
  get_diam(u, v, du, dv, 0);
  while (u != v) {
    if (DEBUG) {
      REP(i, 0, n) {
	REP(j, 0, edges[i].size()) {
	  int w = edges[i][j];
	  cerr << i << "==" << w << endl;
	}
      }
      DEBUGP(u);
      DEBUGP(v);
    }
    cout << "? " << (u + 1) << " " << (v + 1) << endl;
    int ans;
    cin >> ans;
    int piv = -1;
    if (ans == 0) {
      int half_d = du[v] / 2;
      int half = path_recover(du, v, half_d);
      int half_u = path_recover(du, v, half_d - 1);
      int half_v = path_recover(du, v, half_d + 1);
      if (DEBUG) {
	DEBUGP(half_u);
	DEBUGP(half_v);
      }
      set<int> forbid;
      forbid.insert(half_u);
      forbid.insert(half_v);
      cut(half, -1, forbid);
      piv = half;
    } else {
      if (ans == v + 1) {
	swap(u, v);
	swap(du, dv);
      }
      assert (ans == u + 1);
      assert (du[u] == 0);
      int half_d = (du[v] - 1) / 2;
      int half = path_recover(du, v, half_d);
      int half_o = path_recover(du, v, half_d + 1);
      set<int> forbid;
      forbid.insert(half_o);
      cut(half, -1, forbid);
      piv = half;
    }
    get_diam(u, v, du, dv, piv);
  }
  cout << "! " << u + 1 << endl;
}
