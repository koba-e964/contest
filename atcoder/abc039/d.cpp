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
typedef pair<int, int> PI;
const double EPS=1e-9;

const int N = 101;
int h, w;
string s[N];

int orig[N][N];
int gen[N][N];

int main(void){
  cin >> h >> w;
  REP(i, 0, h) {
    cin >> s[i];
  }
  REP(i, 0, h) {
    REP(j, 0, w) {
      orig[i][j] = 1;
    }
  }
  REP(i, 0, h) {
    REP(j, 0, w) {
      REP(u, -1, 2) {
	REP(v, -1, 2) {
	  int nx = i + u;
	  int ny = j + v;
	  if (nx < 0 || nx >= h || ny < 0 || ny >= w || s[i][j] == '#') {
	    continue;
	  }
	  orig[nx][ny] = 0;
	}
      }
    }
  }

  REP(i, 0, h) {
    REP(j, 0, w) {
      REP(u, -1, 2) {
	REP(v, -1, 2) {
	  int nx = i + u;
	  int ny = j + v;
	  if (nx < 0 || nx >= h || ny < 0 || ny >= w || orig[i][j] == 0) {
	    continue;
	  }
	  gen[nx][ny] = 1;
	}
      }
    }
  }

  bool ok = 1;
  REP(i, 0, h) {
    REP(j, 0, w) {
      ok &= gen[i][j] == (s[i][j] == '#');
    }
  }

  if (ok) {
    cout << "possible" << endl;
    REP(i, 0, h) {
      REP(j, 0, w) {
	cout << (orig[i][j] ? '#' : '.');
      }
      cout << endl;
    }
  } else {
    cout << "impossible" << endl;
  }
}
