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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const int N = 26;
VI edges[N];

mt19937 mt(0xe869120);

const int H = 100;
int board[H][H];
PI coord[N];

bool dfs(int v, int par) {
  PI cur = coord[v];
  REP(i, 0, edges[v].size()) {
    int w = edges[v][i];
    if (w == par) { continue; }
    bool found = false;
    PI new_co;
    REP(trial, 0, 10) {
      int dir = mt() % 2;
      int delta = (mt() % 8) - 4;
      if (delta >= 0) {
	delta += 2;
      } else {
	delta -= 1;
      }
      // delta in [-5,-2] + [2,5]
      int dx = dir == 0 ? (delta < 0 ? -1 : 1) : 0; // '|'
      int dy = dir == 1 ? (delta < 0 ? -1 : 1) : 0; // '-'
      bool ok = true;
      for (int j = 1; j <= abs(delta); ++j) {
	int nx = cur.first + dx * j;
	int ny = cur.second + dy * j;
	if (nx < 0 || nx >= H || ny < 0 || ny >= H || board[nx][ny] != 0) {
	  ok = false;
	  break;
	}
      }
      if (ok) {
	int ddx[4] = {1, 0, -1, 0};
	int ddy[4] = {0, 1, 0, -1};
	REP(dir, 0, 4) {
	  int nx = cur.first + ddx[dir] + dx * abs(delta);
	  int ny = cur.second + ddy[dir] + dy * abs(delta);
	  if (nx >= 0 && nx < H && ny >= 0 && ny < H) {
	    char c = board[nx][ny];
	    if (c >= 'A' && c <= 'Z') { // a neighboring vertex found
	      ok = false;
	      break;
	    }
	  }
	}
      }
      if (ok) {
	found = true;
	new_co = PI(cur.first + dx * abs(delta), cur.second + dy * abs(delta));
	board[new_co.first][new_co.second] = 'A' + w;
	REP(j, 1, abs(delta)) {
	  board[cur.first + dx * j][cur.second + dy * j] = dir == 0 ? '|' : '-';
	}
	break;
      }
    }
    if (not found) {
      return false;
    }
    coord[w] = new_co;
    bool sub = dfs(w, v);
    if (not sub) {
      return false;
    }
  }
  return true;
}


int main(void) {
  int n;
  cin >> n;
  REP(lament, 0, n - 1) {
    char a, b;
    cin >> a >> b;
    int v = a - 'A';
    int w = b - 'A';
    edges[v].push_back(w);
    edges[w].push_back(v);
  }
  while (true) {
    // GACHA
    REP(i, 0, H) {
      REP(j, 0, H) {
	board[i][j] = 0;
      }
    }
    int init = mt() % n;
    PI co(H / 2, H / 2);
    board[H / 2][H / 2] = 'A' + init;
    coord[init] = co;
    bool res = dfs(init, -1);
    if (res) {
      break;
    }
  }
  cout << H << " " << H << endl;
  REP(i, 0, H) {
    REP(j, 0, H) {
      int v = board[i][j];
      cout << (v ? (char)v : '.');
    }
    cout << endl;
  }
}
