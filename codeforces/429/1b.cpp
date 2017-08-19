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

void fail(void) {
  cout << "-1\n";
  exit(0);
}

int dfs(vector<bool> &vis, const vector<vector<PI> > &edges, int v,
	 vector<bool> &used, const VI &d) {
  int n = vis.size();
  int m = used.size();
  if (vis[v]) {
    return -1;
  }
  vis[v] = true;
  int cur = d[v];
  REP(i, 0, edges[v].size()) {
    PI weid = edges[v][i];
    int w = weid.first;
    int eid = weid.second;
    int res = dfs(vis, edges, w, used, d);
    if (res == -1) { continue; } // visited
    if (res == 1) {
      used[eid] = true;
      cur ^= 1;
    }
  }
  return cur;
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  VI d(n);
  REP(i, 0, n) {
    cin >> d[i];
  }
  vector<vector<PI> > edges(n);
  REP(i, 0, m) {
    int u, v;
    cin >> u >> v;
    u--, v--;
    edges[u].push_back(PI(v, i));
    edges[v].push_back(PI(u, i));
  }
  // contra?
  {
    int tot = 0;
    int indet = 0;
    REP(i, 0, n) {
      if (d[i] == -1) {
	indet += 1;
      } else {
	tot += d[i];
      }
    }
    if (indet == 0 && tot % 2 != 0) {
      fail();
    }
    // Choose d[i] != -1 appropriately
    REP(i, 0, n) {
      if (d[i] == -1) {
	int val = indet == 1 ? tot % 2 : 0;
	d[i] = val;
	indet -= 1;
      }
    }
    assert (indet == 0);
  }
  // Make a spanning tree
  vector<bool> vis(n);
  vector<bool> used(m, false);
  dfs(vis, edges, 0, used, d);
  VI ue;
  REP(i, 0, m) {
    if (used[i]) {
      ue.push_back(i);
    }
  }
  cout << ue.size() << "\n";
  REP(i, 0, ue.size()) {
    cout << ue[i] + 1 << "\n";
  }
}
