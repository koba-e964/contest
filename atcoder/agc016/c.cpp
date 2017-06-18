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

vector<VI> calc(int hh, int h, int ww, int w) {
  assert (hh % h != 0);
  int x = hh / h;
  // find a, b s.t. a(h - 1) - b < 0, a * hh - (b + a) * x > 0
  int lo = 2 * x * (h - 1);
  int hi = 2 * (hh - x);
  int a = 2 * x;
  int b = lo + 1;
  assert (lo < hi);
  vector<VI> ret(hh, VI(ww, -1));
  REP(i, 0, hh) {
    REP(j, 0, ww) {
      ret[i][j] = i % h == h - 1 ? -b : a;
    }
  }
  return ret;
}

int main(void){
  int hh, ww, h, w;
  cin >> hh >> ww >> h >> w;
  if (hh % h == 0 && ww % w == 0) {
    cout << "No" << endl;
    return 0;
  }
  cout << "Yes" << endl;
  bool flip = false;
  vector<VI> mat;
  if (hh % h == 0) {
    mat = calc(ww, w, hh, h);
    flip = true;
  } else {
    mat = calc(hh, h, ww, w);
  }
  REP(i, 0, hh) {
    REP(j, 0, ww) {
      int num = !flip ? mat[i][j] : mat[j][i];
      cout << num << (j == ww - 1 ? "\n" : " ");
    }
  }
}
