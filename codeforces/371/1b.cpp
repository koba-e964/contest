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

map<pair<PI, PI>, int> memo;
int query(int a, int b, int x, int y) {
  if (a > b || x > y) {
    return 0;
  }
  if (memo.count(pair<PI, PI>(PI(a, b), PI(x, y)))) {
    return memo[pair<PI, PI>(PI(a, b), PI(x, y))];
  }
  cout << "? " << a << " " << x << " " << b << " " << y << endl;
  fflush(stdout);
  int ret;
  cin >> ret;
  memo[pair<PI, PI>(PI(a, b), PI(x, y))] = ret;
  return ret;
}
/*
 * dir = 0: [lo, mid] is checked, dir = 1: [mid, hi] is checked
 */
int bisect(int ty, int lo, int hi, int x, int y, int cnt, int dir) {
  int oldlo = lo;
  int oldhi = hi;
  if (dir) {
    hi ++;
  } else {
    lo--;
  }
  while (hi > lo) {
    int mid = (hi + lo + (dir ? -1 : 1)) / 2;
    int down = dir ? mid : oldlo;
    int up = dir ? oldhi : mid;
    if (query(ty ? x : down, ty ? y : up, ty ? down : x, ty ? up : y) >= cnt) {
      if (dir) {
	lo = mid + 1;
      }	else {
	hi = mid - 1;
      }
    } else {
      if (dir) {
	hi = mid;
      } else {
	lo = mid;
      }
    }
    // cerr << "ty = " << ty << ", dir" << dir << " " << lo << " " << hi << endl;
  }
  return lo + (dir ? (-1) : 1);
}

pair<PI, PI> tt(int n, int ty) {
  int bi1 = bisect(ty, 1, n, 1, n, 1, 0);
  int bi2 = bisect(ty, 1, n, 1, n, 2, 0);
  int anti1 = bisect(ty, 1, n, 1, n, 1, 1);
  int anti2 = bisect(ty, 1, n, 1, n, 2, 1);
  return pair<PI, PI>(PI(anti2, bi1), PI(anti1, bi2));
}

int main(void){
  int n;
  cin >> n;
  pair<PI, PI> row = tt(n, 0);
  pair<PI, PI> col = tt(n, 1);
  int ninty = -1;
  if (row.first.second < row.second.first) {
    ninty = 0;
  }
  if (col.first.second < col.second.first) {
    ninty = 1;
  }
  assert (ninty >= 0);
  int ty = 1 - ninty;
  if (ninty == 0) {
    // col
    int down = row.first.first;
    int up = row.first.second;
    int s1 = bisect(ty, 1, n, down, up, 1, 1);
    int s2 = bisect(ty, 1, n, down, up, 1, 0);
    down = row.second.first;
    up = row.second.second;
    int s3 = bisect(ty, 1, n, down, up, 1, 1);
    int s4 = bisect(ty, 1, n, down, up, 1, 0);
    col.first = PI(s1, s2);
    col.second = PI(s3, s4);
  } else {
    int down = col.first.first;
    int up = col.first.second;
    int s1 = bisect(ty, 1, n, down, up, 1, 1);
    int s2 = bisect(ty, 1, n, down, up, 1, 0);
    down = col.second.first;
    up = col.second.second;
    int s3 = bisect(ty, 1, n, down, up, 1, 1);
    int s4 = bisect(ty, 1, n, down, up, 1, 0);
    row.first = PI(s1, s2);
    row.second = PI(s3, s4);
  }
  cout << "! " <<
    row.first.first << " " << col.first.first << " " <<
    row.first.second << " " << col.first.second << " " <<
    row.second.first << " " << col.second.first << " " <<
    row.second.second << " " << col.second.second << endl;
  fflush(stdout);
  return 0;
}
