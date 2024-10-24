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

// (0, [i, j]): i 行目と j 列目の和集合
// (1, [i]): i 行目
// (2, [i]): i 列目
// (3, [i, j, ...]): 定数個の点 (i, j), ...
// (4, []): 全体集合
pair<int, VI> intersection(const vector<PI> &pts) {
  if (pts.size() == 0) {
    return pair<int, VI>(4, VI());
  }
  if (pts.size() == 1) {
    return pair<int, VI>(0, VI({pts[0].first, pts[0].second}));
  }
  pair<int, VI> cur = pair<int, VI>(0, VI({pts[0].first, pts[0].second}));
  REP(i, 1, pts.size()) {
    pair<int, VI> nxt = pair<int, VI>(0, VI({pts[i].first, pts[i].second}));
    int x = pts[i].first, y = pts[i].second;
    if (cur.first == 0) {
      if (cur.second[0] == x) {
        nxt = pair<int, VI>(1, VI({x}));
      } else if (cur.second[1] == y) {
        nxt = pair<int, VI>(2, VI({y}));
      } else {
        nxt = pair<int, VI>(3, VI({x, cur.second[1], cur.second[0], y}));
      }
    } else if (cur.first == 1) {
      if (x == cur.second[0]) {
        nxt = cur;
      } else {
        nxt = pair<int, VI>(3, VI({cur.second[0], y}));
      }
    } else if (cur.first == 2) {
      if (y == cur.second[1]) {
        nxt = cur;
      } else {
        nxt = pair<int, VI>(3, VI({x, cur.second[1]}));
      }
    } else {
      assert (cur.first == 3);
      VI new_pts;
      REP(k, 0, cur.second.size() / 2) {
        int px = cur.second[2 * k], py = cur.second[2 * k + 1];
        if (px == x || py == y) {
          new_pts.push_back(px);
          new_pts.push_back(py);
        }
      }
      nxt = pair<int, VI>(3, new_pts);
    }
    cur = nxt;
  }
  return cur;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int h, w;
  cin >> h >> w;
  vector<VI> p(h, VI(w));
  vector<vector<PI>> occ(h * w);
  REP(i, 0, h) {
    REP(j, 0, w) {
      cin >> p[i][j];
      occ[p[i][j]].push_back(PI(i, j));
    }
  }
  vector<bool> rows(h), cols(w);
  vector<VI> ans(h, VI(w, h * w));
  REP(mex, 0, h * w) {
    pair<int, VI> inter = intersection(occ[mex]);
    if (false) {
      cerr << mex << ": " << inter.first << endl;
      for (int x: inter.second) cerr << x << " ";
      cerr << endl;
    }
    if (inter.first == 4) {
      REP(i, 0, h) {
        if (rows[i]) {
          continue;
        }
        REP(j, 0, w) {
          if (ans[i][j] == h * w) ans[i][j] = mex;
          cols[j] = true;
        }
        rows[i] = true;
      }
      break;
    }
    if (inter.first == 0) {
      int x = inter.second[0], y = inter.second[1];
      if (!cols[y])
        REP(i, 0, h) if (ans[i][y] == h * w) ans[i][y] = mex;
      if (!rows[x])
        REP(j, 0, w) if (ans[x][j] == h * w) ans[x][j] = mex;
      rows[x] = true;
      cols[y] = true;
    } else if (inter.first == 1) {
      int x = inter.second[0];
      if (rows[x]) {
        continue;
      }
      REP(j, 0, w) if (ans[x][j] == h * w) ans[x][j] = mex;
      rows[x] = true;
    } else if (inter.first == 2) {
      int y = inter.second[0];
      if (cols[y]) {
        continue;
      }
      REP(i, 0, h) if (ans[i][y] == h * w) ans[i][y] = mex;
      cols[y] = true;
    } else {
      assert (inter.first == 3);
      REP(k, 0, inter.second.size() / 2) {
        int x = inter.second[2 * k], y = inter.second[2 * k + 1];
        if (rows[x] || cols[y]) {
          continue;
        }
        if (ans[x][y] == h * w) ans[x][y] = mex;
      }
    }
  }
  REP(i, 0, h) {
    REP(j, 0, w) {
      cout << ans[i][j] << (j == w - 1 ? "\n" : " ");
    }
  }
}
