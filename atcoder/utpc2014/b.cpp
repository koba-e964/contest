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



int main(void){
  double x, y;
  cin >> x >> y;
  int bx = floor(x);
  int by = floor(y);
  x -= bx; y -= by;
  int cx = 1;
  int cy = 0;
  if (y < 1e-9) {
    cy = 1;
  }
  PI r1(floor(1000 * x + 0.5 + bx), floor(1000 * y + 0.5 + by));
  if (x < 1e-9 && y < 1e-9) {
    r1 = PI(1 + bx, by);
  }
  PI r2(floor(1000 * (x - cx) + cx + 0.5 + bx),
	floor(1000 * (y - cy) + cy + 0.5 + by));
  PI r3(cx + bx, cy + by);
  cout << bx << " " << by << " " << r1.first << " " << r1.second << endl;
  cout << r2.first << " " << r2.second << " " << r3.first << " " << r3.second << endl;
}
