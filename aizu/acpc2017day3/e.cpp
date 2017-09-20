#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const int N = 4096;
int bit[2][N][N];

int accum(int kind, int x, int y) {
  int tot = 0;
  while (x > 0) {
    int yy = y;
    while (yy > 0) {
      tot += bit[kind][x][yy];
      yy &= yy - 1;
    }
    x &= x - 1;
  }
  return tot;
}

void update(int kind, int x, int y, int v) {
  while (x < N) {
    int yy = y;
    while (yy < N) {
      bit[kind][x][yy] += v;
      yy += yy & (-yy);
    }
    x += x & (-x);
  }
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int h, w, q;
  ll t_bake;
  cin >> h >> w >> t_bake >> q;
  queue<pair<ll, PI> > que; 
  REP(benedicta, 0, q) {
    ll t;
    int c, h, w;
    cin >> t >> c >> h >> w;
    while (not que.empty() && que.front().first <= t) {
      PI top = que.front().second;
      que.pop();
      int x = top.first;
      int y = top.second;
      update(1, x, y, 1);
      update(0, x, y, -1);
    }
    if (c == 0) {
      // set
      que.push(make_pair(t + t_bake, PI(h, w)));
      update(0, h, w, 1);
    } else if (c == 1) {
      // pick
      int cnt = accum(1, h, w) - accum(1, h, w - 1)
	- accum(1, h - 1, w) + accum(1, h - 1, w - 1);
      assert (0 <= cnt && cnt <= 1);
      if (cnt == 1) {
	update(1, h, w, -1);
      }
    } else if (c == 2) {
      // count
      int h2, w2;
      cin >> h2 >> w2;
      int b[2];
      REP(k, 0, 2) {
	b[k] = accum(k, h2, w2) - accum(k, h2, w - 1)
	  - accum(k, h - 1, w2) + accum(k, h - 1, w - 1);
      }
      cout << b[1] << " " << b[0] << "\n";
    }
  }
}
