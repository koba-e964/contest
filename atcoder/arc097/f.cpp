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
typedef pair<int, int> PI;

const int N = 100100;
int n;
VI g[N];
int deg[N];
bool alive[N];

VI gdist(int v, const VI &vals) {
  VI ret(n, -1);
  queue<PI> que;
  que.push(PI(v, vals[v]));
  while (not que.empty()) {
    PI t = que.front(); que.pop();
    int w = t.first;
    int d = t.second;
    if (ret[w] >= 0) continue;
    ret[w] = d;
    REP(i, 0, g[w].size()) {
      int u = g[w][i];
      que.push(PI(u, d + vals[u]));
    }
  }
  return ret;
}

string c;

VI vert;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n;
  REP(i, 0, n - 1) {
    int x, y;
    cin >> x >> y;
    x--, y--;
    g[x].push_back(y);
    g[y].push_back(x);
  }
  cin >> c;
  {
    queue<int> que;
    REP(i, 0, n) {
      alive[i] = true;
      deg[i] = g[i].size();
      if (g[i].size() == 1) que.push(i);
    }
    while (not que.empty()) {
      int v = que.front(); que.pop();
      if (c[v] == 'B') {
	// delete
	int nxt = -1;
	REP(i, 0, g[v].size()) {
	  if (alive[g[v][i]]) {
	    nxt = g[v][i];
	    break;
	  }
	}
	alive[v] = false;
	deg[nxt]--;
	if (deg[nxt] == 1) que.push(nxt);
      }
    }
    // deletes all unnecessary edges
    REP(i, 0, n) {
      VI cp;
      REP(j, 0, g[i].size()) {
	int w = g[i][j];
	if (not alive[w]) continue;
	cp.push_back(w);
      }
      g[i] = cp;
    }
    REP(i, 0, n) if (alive[i]) vert.push_back(i);
  }
  if (DEBUG) {
    REP(i, 0, n) {
      if (alive[i]) {
	REP(j, 0, g[i].size()) {
	  int w = g[i][j];
	  cerr << " (" << i << "," << w << ")";
	}
	cerr << endl;
      }
    }
  }
  VI vals(n);
  for (int i: vert) {
    vals[i] = (c[i] == 'W') ^ (deg[i] % 2);
  }
  int idx = -1;
  for (int i: vert) {
    if (alive[i] && c[i] == 'W') idx = i;
  }
  if (idx < 0) {
    cout << 0 << endl;
    return 0;
  }
  if (vert.size() <= 1) {
    cout << vert.size() << endl;
    return 0;
  }
  assert (idx >= 0);
  PI ma1(-1, -1);
  VI d1 = gdist(idx, vals);
  for (int i: vert) {
    if (c[i] == 'W') {
      ma1 = max(ma1, PI(d1[i], i));
    }
  }
  PI ma2(-1, -1);
  VI d2 = gdist(ma1.second, vals);
  for (int i: vert) {
    if (c[i] == 'W') {
      ma2 = max(ma2, PI(d2[i], i));
    }
  }
  if (DEBUG) {
    cerr << "vals:";
    REP(i, 0, n) cerr << " " << vals[i];
    cerr << endl;
    cerr << "d2:";
    REP(i, 0, n) cerr << " " << d2[i];
    cerr << endl;
  }
  int diam = ma2.first;
  int base = 2 * (vert.size() - 1);
  for (int i: vert) {
    base += vals[i];
  }
  DEBUGP(base);
  DEBUGP(diam);
  cout << base - 2 * diam << endl;
}
