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



int main(void){
  int x, y;
  string w;
  cin >> x >> y >> w;
  x--, y--;
  string s[9];
  REP(i, 0, 9) {
    cin >> s[i];
  }
  int dx = 0, dy = 0;
  REP(i, 0, w.length()) {
    switch(w[i]) {
    case 'L':
      x = 16 - x;
      dx = 1;
      break;
    case 'U':
      y = 16 - y;
      dy = 1;
      break;
    case 'R':
      dx = 1;
      break;
    case 'D':
      dy = 1;
      break;
    default:
      break;
    }
  }
  REP(i, 0, 4) {
    int nx = x >= 9 ? 16 - x : x;
    int ny = y >= 9 ? 16 - y : y;
    cout << s[ny][nx];
    x += dx;
    y += dy;
    x %= 16;
    y %= 16;
  }
  cout << endl;
}
