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

const int N = 400020;
const int B = 200010;

const int DEBUG = 0;

set<int> cols[N];
set<int> rows[N];

bool ok(const set<int> &s, int x, int y) {
  return distance(s.lower_bound(x), s.lower_bound(y)) == y - x;
}

PI next_pos(int x, int y, int dx, int dy) {
  set<int> *ptr;
  int dv;
  int v;
  cols[x + B].insert(y);
  rows[y + B].insert(x);
  if (dx == 0) { // column
    ptr = &cols[x + B];
    dv = dy;
    v = y;
  } else {
    ptr = &rows[y + B];
    dv = dx;
    v = x;
  }
  set<int> &dat = *ptr;
  int res;
  if (dv == 1) {
    int lo = v;
    int hi = N - B;
    while (hi - lo >= 2) {
      int mid = (hi + lo) / 2;
      if (ok(dat, v + 1, mid + 1)) {
	lo = mid;
      } else {
	hi = mid;
      }
    }
    res = hi;
  } else {
    int lo = -B;
    int hi = v;
    while (hi - lo >= 2) {
      int mid = (hi + lo) / 2;
      if (ok(dat, mid, v)) {
	hi = mid;
      } else {
	lo = mid;
      }
    }
    res = lo;
  }
  if (dx == 0) {
    return PI(x,res);
  }
  return PI(res, y);
}


int main(void){
  int k;
  string s;
  cin >> k >> s;
  int x = 0, y = 0;
  REP(i, 0, k) {
    if(DEBUG) {
      cout << "time = " << i << ", x ="  << x << ", y=" << y << endl;
    }
    int dx, dy;
    switch(s[i]) {
    case 'L':
      dx = -1, dy=0;break;
    case 'R':
      dx=1,dy=0;break;
    case 'U':
      dx=0,dy=1;break;
    default:
      dx=0,dy=-1;
    }
    PI res = next_pos(x, y,dx, dy);
    x = res.first;
    y = res.second;
  }
  cout << x << " " << y << endl;
}
