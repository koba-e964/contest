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
typedef pair<PI, PI> PPIPI;
const ll mod = 1e9 + 7;

vector<PPIPI> ops;

void output(void) {
  cout << ops.size() << "\n";
  REP(i, 0, ops.size()) {
    PPIPI op = ops[i];
    PI s = op.first;
    PI t = op.second;
    cout << s.first << " " << s.second << " " << t.first << " " << t.second << "\n";
  }
  exit(0);
}

void make2(VI x, VI y, VI gx, VI gy) {
  assert (gx[0] != gx[1]);
  if (x[0] == x[1]) {
    int idx;
    if (x[0] == gx[0]) idx = 1;
    else idx = 0;
    ops.push_back(PPIPI(PI(x[idx], y[idx]), PI(3 - x[idx], y[idx])));
    x[idx] = 3 - x[idx];
  }
  if (x[0] != gx[0]) {
    assert (x[1] != gx[1]);
    REP(i, 0, 2) {
      if (y[i] != i + 1) {
	ops.push_back(PPIPI(PI(x[i], y[i]), PI(x[i], i + 1)));
	y[i] = i + 1;
      }
    }
    REP(i, 0, 2) {
      ops.push_back(PPIPI(PI(x[i], y[i]), PI(3 - x[i], y[i])));
      x[i] = 3 - x[i];
    }
  }
  REP(i, 0, 2) {
    if (y[i] != gy[i]) {
      ops.push_back(PPIPI(PI(x[i], y[i]), PI(x[i], 3 - y[i])));
    }
  }
}

bool no_intersect(const vector<PI> &t) {
  REP(i, 0, t.size()) {
    REP(j, 0, i) {
      if (t[i] == t[j]) return false;
    }
  }
  return true;
}

void get_move(const vector<PI> &cur, const vector<PI> &nxt) {
  int cnt = 0;
  REP(i, 0, cur.size()) {
    if (cur[i] != nxt[i]) {
      cnt++;
      ops.push_back(PPIPI(cur[i], nxt[i]));
    }
  }
  assert (cnt == 1);
}

void exhaust3(int sz,VI x, VI y, VI gx, VI gy) {
  map<vector<PI>, int> dist;
  queue<pair<vector<PI>,int> > que;
  int n = x.size();
  vector<PI> st(n);
  REP(i, 0, n) st[i] = PI(gx[i], gy[i]);
  que.push(make_pair(st, 0));
  int dx[4] = {1, 0, -1, 0};
  int dy[4] = {0, 1, 0, -1};
  map<vector<PI>, vector<vector<PI> > > adj;
  while (not que.empty()) {
    pair<vector<PI>, int> curd = que.front(); que.pop();
    vector<PI> cur = curd.first;
    int dis = curd.second;
    if (dist.count(cur)) continue;
    dist[cur] = dis;
    REP(i, 0, n) {
      REP(d, 0, 4) {
	vector<PI> nxt(cur);
	nxt[i].first += dx[d];
	nxt[i].second += dy[d];
	if (nxt[i].first < 1 || nxt[i].first > sz || nxt[i].second < 1 || nxt[i].second > sz) continue;
	if (not no_intersect(nxt)) continue;
	adj[nxt].push_back(cur);
	que.push(make_pair(nxt, dis + 1));
      }
    }
  }
  vector<PI> cur(n);
  REP(i, 0, n) cur[i] = PI(x[i], y[i]);
  int rem = dist[cur];
  while (rem > 0) {
    for (vector<PI> &nxt: adj[cur]) {
      if (dist[nxt] == rem - 1) {
	get_move(cur, nxt);
	cur = nxt;
	rem--;
	break;
      }
    }
  }
}


void make4(int n, VI x, VI y, VI gx, VI gy) {
  vector<vector<PI> > gxset(n + 1);
  REP(i, 0, n) gxset[gx[i]].push_back(PI(gy[i], i));
  REP(i, 1, n + 1) {
    assert ((int) gxset[i].size() < n - 1);
  }
  vector<vector<PI> > xset(n + 1);
  REP(i, 0, n) xset[gx[i]].push_back(PI(gy[i], i));
  REP(i, 1, n + 1) {
    REP(j, 0, xset[i].size()) {
      int idx = xset[i][j].second;
      int dst = j + 1;
      for (int k = xset[i][j].first; k > dst; --k) {
	ops.push_back(PPIPI(PI(i, k), PI(i, k - 1)));
      }
      y[idx] = dst;
    }
  }
  REP(i, 0, n) {
    if (y[i] == n) {
      int nxt = x[i] == 1 ? 2 : x[i] - 1;
      x[i] = nxt;
      y[i] = 1;
      xset[nxt].push_back(PI(1, i));
    }
  }
  queue<int> que;
  REP(i, 0, n) if (y[i] > 1) que.push(i);
  set<int> vacant;
  REP(i, 1, n + 1) if (xset[i].size() == 0) vacant.insert(i);
  while (not que.empty()) {
    int idx = que.front(); que.pop();
    int dstx = *vacant.begin();
    PI t(x[idx], y[idx]);
    for (int i = t.second; i < n; ++i) {
      ops.push_back(PPIPI(t, PI(t.first, t.second + 1)));
      t.second++;
    }
    int dx = dstx < x[idx] ? -1 : 1;
    for (int i = x[idx]; i != dstx; i += dx) {
      ops.push_back(PPIPI(t, PI(t.first + dx, t.second)));
      t.first += dx;
    }
    for (int i = n; i > 1; --i) {
      ops.push_back(PPIPI(t, PI(t.first, t.second - 1)));
      t.second--;
    }
  }
  REP(i, 1, n + 1) {
    sort(gxset[i].rbegin(), gxset[i].rend());
    for (PI d: gxset[i]) {
      int idx = d.second;
      int dsty = d.first;
      ops.push_back(PPIPI(PI(x[idx], 1), PI(x[idx], 2)));
      int dx = i < x[idx] ? -1 : 1;
      for (int j = x[idx]; j != i; j += dx) {
	ops.push_back(PPIPI(PI(j, 2), PI(j + dx, 2)));
      }
      for (int j = 2; j < dsty; ++j) {
	ops.push_back(PPIPI(PI(i, j), PI(i, j + 1)));
      }
    }
  }
  REP(i, 1, n + 1) {
    // TODO
  }
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  VI x(m), y(m);
  VI gx(m), gy(m);
  REP(i, 0, m) cin >> x[i] >> y[i];
  REP(i, 0, m) cin >> gx[i] >> gy[i];
  if (n == 1) {
    output();
  }
  if (n == 2 && 0) {
    if (m == 1) {
      // just move!
      if (x[0] != gx[0] && y[0] != gy[0]) {
	ops.push_back(PPIPI(PI(x[0], y[0]), PI(gx[0], y[0])));
	ops.push_back(PPIPI(PI(gx[0], y[0]), PI(gx[0], gy[0])));
      } else if (x[0] != gx[0] && y[0] == gy[0]) {
	ops.push_back(PPIPI(PI(x[0], y[0]), PI(gx[0], y[0])));
      } else if (x[0] == gx[0] && y[0] != gy[0]) {
	ops.push_back(PPIPI(PI(x[0], y[0]), PI(x[0], gy[0])));
      }
      output();
    }
    assert (m == 2);
    if (gx[0] == gx[1]) {
      REP(i, 0, n) {
	swap(x[i], y[i]);
	swap(gx[i], gy[i]);
      }
      make2(x, y, gx, gy);
      REP(i, 0, n) {
	swap(x[i], y[i]);
	swap(gx[i], gy[i]);
      }
      REP(i, 0, ops.size()) {
	swap(ops[i].first.first, ops[i].first.second);
	swap(ops[i].second.first, ops[i].second.second);
      }
    } else {
      make2(x, y, gx, gy);
    }
    output();
  }
  if (n <= 3) {
    exhaust3(n, x, y, gx, gy);
    output();
  }
  assert (n >= 4);
  vector<vector<PI> > xset(n + 1);
  REP(i, 0, n) xset[gx[i]].push_back(PI(gy[i], i));
  bool ok = true;
  REP(i, 1, n + 1) {
    if ((int) xset[i].size() >= n - 1) ok = false;
  }
  if (not ok) {
    make4(n, y, x, gy, gx);
    REP(i, 0, ops.size()) {
      swap(ops[i].first.first, ops[i].first.second);
      swap(ops[i].second.first, ops[i].second.second);
    }
  } else {
    make4(n, x, y, gx, gy);
  }
  output();
}
