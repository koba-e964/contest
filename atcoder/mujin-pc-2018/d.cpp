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
const ll mod = 1e9 + 7;

const int N = 1000;
PI nxt[N][N];
bool loop[N][N], doom[N][N];


int rev[N];
void init(void) {
  REP(i, 1, N) {
    int v = i;
    int res = 0;
    while (v > 0) {
      res *= 10;
      res += v % 10;
      v /= 10;
    }
    rev[i] = res;
  }
}

int main(void) {
  init();
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  REP(i, 1, N) {
    REP(j, 1, N) {
      int x = i, y =j;
      if (i < j) {
	x = rev[x];
      } else {
	y = rev[y];
      }
      if (x < y) {
	y = abs(x - y);
      } else {
	x = abs(x - y);
      }
      nxt[i][j] = PI(x, y);
    }
  }
  REP(i, 1, N) {
    REP(j, 1, N) {
      int x = i, y = j;
      bool is_doom = false;
      while (1) {
	loop[x][y] = 1;
	PI n = nxt[x][y];
	x = n.first; y = n.second;
	if (x == 0 || y == 0 || doom[x][y]) {
	  is_doom = true;
	  break;
	}
	if (loop[x][y]) {
	  break;
	}
      }
      doom[x][y] = is_doom;
      if (is_doom) {
	x = i, y = j;
	while (1) {
	  loop[x][y] = 0;
	  PI n = nxt[x][y];
	  x = n.first; y = n.second;
	  if (x == 0 || y == 0) {
	    is_doom = true;
	    break;
	  }
	}
      }
    }
  }
  int ans = 0;
  REP(i, 1, n + 1) {
    REP(j, 1, m + 1) {
      if (DEBUG) {
	cerr << i << " " << j << " " << nxt[i][j].first << " " << nxt[i][j].second << endl;
      }
      ans += loop[i][j];
    }
  }
  cout << ans << endl;
}
