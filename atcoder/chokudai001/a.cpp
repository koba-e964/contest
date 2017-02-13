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
      if (trial % 10 == 9 || avrg_uncertainty <= 50) {
	cout << " [" << trial + 1 << "] estimated avrg = " << avrg << " \\pm "
	   << avrg_uncertainty << endl;
	if (trial >= 10 && avrg_uncertainty <= 50) {
	  break;
	}
      }
    }
  }
}

vector<PI> solve(vector<VI> a) {
  vector<PI> ret;
  while (true) {
    int ma = 0;
    int maxx = -1, maxy = -1;
    REP(i, 0, W) {
      REP(j, 0, W) {
	if (ma < a[i][j]) {
	  maxx = i;
	  maxy = j;
	}
      }
    }
    if (maxx == -1 && maxy == -1) {
      break;
    }
    int x = maxx;
    int y = maxy;
    while (true) {
      //cerr << "x = " << x << ", y = " << y << " a[x][y] = " << a[x][y] << endl;
      a[x][y]--;
      ret.push_back(PI(x, y));
      int tmp = a[x][y];
      if (tmp <= 0) {
	break;
      }
      int dx[4] = {1, 0, -1, 0};
      int dy[4] = {0, 1, 0, -1};
      int cx = -1;
      int cy = -1;
      REP(d, 0, 4) {
	int nx = x + dx[d];
	int ny = y + dy[d];
	if (nx < 0 || nx >= W || ny < 0 || ny >= W) {
	  continue;
	}
	if (tmp == a[nx][ny]) {
	  cx = nx;
	  cy = ny;
	  break;
	}
      }
      if (cx >= 0 && cy >= 0) {
	x = cx; y = cy;
      } else {
	break;
      }
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
