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
typedef pair<PI, int> PPII;

const int N = 510;
string s[N];
const int inf = 1e8;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  int gx = -1, gy = -1;
  REP(i, 0, n) {
    cin >> s[i];
    REP(j, 0, m) {
      if (s[i][j] == 'C') {
	gx = i, gy = j;
      }
    }
  }
  
  queue<PPII> que;
  vector<VI> tbl(n, VI(m, inf));
  que.push(PPII(PI(gx, gy), 0));
  while (not que.empty()) {
    PPII t = que.front(); que.pop();
    int x = t.first.first;
    int y = t.first.second;
    int dist = t.second;
    if (s[x][y] == '#') {
      continue;
    }
    if (dist >= tbl[x][y]) {
      continue;
    }
    tbl[x][y] = dist;
    int dx[4] = {1, 0, -1, 0};
    int dy[4] = {0, 1, 0, -1};
    REP(d, 0, 4) {
      int nx = x + dx[d];
      int ny = y + dy[d];
      que.push(PPII(PI(nx, ny), dist + 1));
    }
  }
  int sum = 0;
  REP(i, 0, n) {
    REP(j, 0, m) {
      if (s[i][j] == 'S') {
	sum += tbl[i][j];
      }
      if (s[i][j] == 'G') {
	sum += tbl[i][j];
      }
    }
  }
  cout << (sum >= inf ? -1 : sum) << "\n";
}
