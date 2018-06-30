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

const int N = 2100;
int a[N][N], dif[N][N];
int h, w;
set<PI> on;

void flip(int x, int y) {
  if (x < 0 || y < 0 || x >= h || y >= w) return;
  if (dif[x][y]) {
    dif[x][y] = 0;
    on.erase(PI(x, y));
    return;
  }
  dif[x][y] = 1;
  if (x >= 1 && y >= 1) {
    on.insert(PI(x, y));
  }
}

void debug(void) {
  REP(i, 0, h) {
    REP(j, 0, w) cerr << dif[i][j];
    cerr << endl;
  }
  for (PI e: on) {
    cerr << " (" << e.first << "," << e.second << ")";
  }
  cerr << endl;
}

bool check(void) {
  if (on.size() > 4) return false;
  if (on.size() <= 1) return true;
  if (on.size() <= 2) {
    // Do they share row or column?
    vector<PI> pack(on.begin(), on.end());
    return pack[0].first == pack[1].first || pack[0].second == pack[1].second;
  }
  if (on.size() == 4) {
    vector<PI> pack(on.begin(), on.end());
    sort(pack.begin(), pack.end());
    int x1 = pack[0].first, x2 = pack[2].first;
    int y1 = pack[0].second, y2 = pack[1].second;
    return pack[1] == PI(x1, y2) && pack[2] == PI(x2, y1) && pack[3] == PI(x2, y2);
  }
  assert (on.size() == 3);
  return false;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int q;
  cin >> h >> w >> q;
  vector<string> s(h);
  REP(i, 0, h) cin >> s[i];
  REP(i, 0, h) {
    REP(j, 0, w) {
      a[i][j] = s[i][j] == '#' ? 1 : 0;
    }
  }
  REP(i, 0, h) {
    REP(j, 0, w) {
      int tmp = a[i][j];
      if (j > 0) tmp ^= a[i][j - 1];
      if (i > 0) tmp ^= a[i - 1][j];
      if (i > 0 && j > 0) tmp ^= a[i - 1][j - 1];
      if (tmp) flip(i, j);
    }
  }
  REP(i, -1, q - 1) {
    if (i >= 0) {
      int r, c;
      cin >> r >> c;
      r--, c--;
      flip(r, c);
      flip(r, c + 1);
      flip(r + 1, c);
      flip(r + 1, c + 1);
    }
    cout << (check() ? "Yes" : "No") << endl;
    // debug();
  }
}
