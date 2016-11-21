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
const ll mod = 1e9 + 7;


int query(int a, int b, int c, int d) {
  cout << "? " << a << " " << b << " " << c << " " << d << "\n";
  int r;
  cin >> r;
  return r;
}

const int H = 50;
int board[H][H] = {};
// [a, a + x) * [b, b + y)
void solve(int a, int b, int x, int y, int dir, int tot) {
  //cerr << "solve:" << a << " " << b << " " << x << " " << y << " " << dir << " " << tot << "\n";
  if (x * y == tot) {
    REP(i, a, a + x) {
      REP(j, b, b + y) {
	board[i][j] = 1;
      }
    }
    return;
  }
  if (tot == 0) {
    return;
  }
  assert (x >= 2 || y >= 2);
  if ((dir == 0 && x == 1) || (dir == 1 && y == 1)) {
    dir = 1 - dir;
  }
  int nx = dir == 0 ? x / 2 : x;
  int ny = dir == 1 ? y / 2 : y;
  int dx = dir == 0 ? x / 2 : 0;
  int dy = dir == 1 ? y / 2 : 0;
  int res_half = query(a, b, a + nx - 1, b + ny - 1);
  solve(a, b, nx, ny, 1 - dir, res_half);
  solve(a + dx, b + dy, x - dx, y - dy, 1 - dir, tot - res_half);
}

int main(void){
  ios::sync_with_stdio(false);
  int h, w, n;
  ll k;
  cin >> h >> w >> n >> k;
  solve(0, 0, h, w, 0, n);

  ll sum = 0;
  ll cur = 1;
  REP(i, 0, h) {
    REP(j, 0, w) {
      int res = board[i][j];
      sum = (sum + res * cur) % k;
      cur = cur * 2 % k;
    }
  }
  cout << "! " << sum << "\n";
}
