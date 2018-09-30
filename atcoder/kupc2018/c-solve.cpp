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

bool board[9][9];

bool verify(void) {
  REP(i, 0, 9) {
    int cur = 0;
    REP(j, 0, 9) {
      if (board[i][j]) cur = 0;
      else cur++;
      if (cur >= 7) return false;
    }
  }
  REP(j, 0, 9) {
    int cur = 0;
    REP(i, 0, 9) {
      if (board[i][j]) cur = 0;
      else cur++;
      if (cur >= 7) return false;
    }
  }
  REP(i, -2, 3) {
    int x = max(0, i);
    int y = max(0, -i);
    int len = 9 - x - y;
    int cur = 0;
    REP(j, 0, len) {
      if (board[x + j][y + j]) cur = 0;
      else cur++;
      if (cur >= 7) return false;
    }
  }
  REP(i, -2, 3) {
    int x = 8 - max(0, i);
    int y = max(0, -i);
    int len = 9 - abs(i);
    int cur = 0;
    REP(j, 0, len) {
      if (board[x - j][y + j]) cur = 0;
      else cur++;
      if (cur >= 7) return false;
    }
  }
  return true;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  VI a(9);
  REP(i, 0, 9) a[i] = i;
  mt19937 mt;
  for (ll seen = 0; seen < 387420489/*9^9*/; ++seen) {
    if (seen % 10000 == 0) cerr << "seen = " << seen << endl;
    REP(i, 0, 9) REP(j, 0, 9) board[i][j] = 0;
    ll v = mt() % 387420489;
    ll id = v;
    REP(i, 0, 9) {
      a[i] = v % 9;
      v /= 9;
    }
    REP(i, 0, 9) board[i][a[i]] = 1;
    REP(i, 0, 81) {
      int x = i / 9, y = i % 9;
      if (board[x][y]) continue;
      REP(j, 0, i) {
        int z = j / 9, w = j % 9;
        if (board[z][w]) continue;
        board[x][y] = 1;
        board[z][w] = 1;
        if (verify()) {
          cout << "board id = " << id << endl;
          REP(u, 0, 9) {
            REP(v, 0, 9) {
              cout << (board[u][v] ? '#' : '.');
            }
            cout << endl;
          }
          return 0;
        }
        board[x][y] = 0;
        board[z][w] = 0;
      }
    }
  }
}
