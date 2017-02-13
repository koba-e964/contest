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

const int W = 30;

int score(vector<VI> &a, const vector<PI> &sol) {
  int cnt = 0;
  int tmp = -1;
  int tmpx = -1, tmpy = -1;
  REP(i, 0, sol.size()) {
    int x = sol[i].first;
    int y = sol[i].second;
    if (tmp == a[x][y] && abs(tmpx - x) + abs(tmpy - y) == 1) {
      cnt += 0;
    } else {
      cnt += 1;
    }
    a[x][y]--;
    tmp = a[x][y];
    tmpx = x;
    tmpy = y;
    assert (a[x][y] >= 0);
  }
  REP(i, 0, W) {
    REP(j, 0, W) {
      assert (a[i][j] == 0);
    }
  }
  return 100000 - cnt;
}

vector<PI> solve(vector<VI>);


void checker(void) {
  srand(0x12312);
  double sum = 0;
  double sq_sum = 0;
  REP(trial, 0, 2000) {
    vector<VI> a(W, VI(W));
    REP(i, 0, W) {
      REP(j, 0, W) {
	a[i][j] = 1 + rand() % 100;
      }
    }
    vector<PI> sol = solve(a);
    int sc = score(a, sol);
    sum += sc;
    sq_sum += (double)sc * (double)sc;
    double avrg = sum / (trial + 1);
    double variance = sq_sum / (trial + 1) - avrg * avrg;
    if (trial >= 9) {
      double avrg_uncertainty = sqrt(variance / trial);
      if (trial % 10 == 9 || avrg_uncertainty <= 40) {
	cout << " [" << trial + 1 << "] estimated avrg = " << avrg << " \\pm "
	   << avrg_uncertainty << endl;
	if (trial >= 10 && avrg_uncertainty <= 40) {
	  cout << " estimated stdev = "
	       << sqrt(variance * (trial + 1) / trial) << endl;
	  break;
	}
      }
    }
  }
}

vector<PI> solve(vector<VI> a) {
  vector<PI> ret;
  const int dx[4] = {1, 0, -1, 0};
  const int dy[4] = {0, 1, 0, -1};
 
  while (true) {
    int ma = 0;
    int maxx = -1, maxy = -1;
    REP(i, 0, W) {
      REP(j, 0, W) {
	if (ma < a[i][j]) {
	  ma = a[i][j];
	  maxx = i;
	  maxy = j;
	}
      }
    }
    if (maxx == -1 && maxy == -1) {
      break;
    }
    // detect the longest path from (maxx, maxy)
    const int inf = 1e8;
    vector<VI> dist(W, VI(W, inf));
    queue<pair<PI, int> > que;
    que.push(make_pair(PI(maxx, maxy), 0));
    while (not que.empty()) {
      pair<PI, int> t = que.front();
      int x = t.first.first;
      int y = t.first.second;
      int c = t.second;
      que.pop();
      if (a[x][y] <= 0) {
	continue;
      }
      if (dist[x][y] <= c) {
	continue;
      }
      dist[x][y] = c;
      REP(d, 0, 4) {
	int nx = x + dx[d];
	int ny = y + dy[d];
	if (nx < 0 || nx >= W || ny < 0 || ny >= W) {
	  continue;
	}
	if (a[x][y] - 1 == a[nx][ny]) {
	  que.push(make_pair(PI(nx, ny), c + 1));
	}
      }
    }
    // Find the longest path from (maxx, maxy)
    int tx = maxx, ty = maxy;
    {
      int ma = 0;
      REP(i, 0, W) {
	REP(j, 0, W) {
	  if (dist[i][j] < inf && ma < dist[i][j]) {
	    ma = dist[i][j];
	    tx = i;
	    ty = j;
	  }
	}
      }
    }
    vector<PI> path; // path recovery
    while (dist[tx][ty] > 0) {
      path.push_back(PI(tx, ty));
      REP(d, 0, 4) {
	int nx = tx + dx[d];
	int ny = ty + dy[d];
	if (nx < 0 || nx >= W || ny < 0 || ny >= W) {
	  continue;
	}
	if (dist[tx][ty] - 1 == dist[nx][ny]) {
	  tx = nx;
	  ty = ny;
	  break;
	}
      }
    }
    path.push_back(PI(tx, ty));
    reverse(path.begin(), path.end());
    int tmp = -1;
    REP(i, 0, path.size()) {
      int x = path[i].first;
      int y = path[i].second;
      if (tmp >= 0 && tmp != a[x][y]) {
	break;
      }
      a[x][y]--;
      tmp = a[x][y];
      ret.push_back(path[i]);
    }
  }
  return ret;
}

int main(void){
#ifdef DEBUG
  checker();
#else
  vector<VI> a(W, VI(W));
  REP(i, 0, W) {
    REP(j, 0, W) {
      cin >> a[i][j];
    }
  }
  vector<PI> result = solve(a);
  REP(i, 0, result.size()) {
    cout << result[i].first + 1 << " "
	 << result[i].second + 1 << endl;
  }
#endif
}
