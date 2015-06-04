#include <algorithm>
#include <bitset>
#include <cassert>
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
  int x1,y1,x2,y2,x3,y3;
  cin >> x1 >> y1 >> x2 >> y2 >> x3 >> y3;
  int d2 = x2 - x1,e2 = y2 - y1, d3 = x3 - x1, e3 = y3 - y1;
  if (d2 * d2 + e2 * e2 > d3 * d3 + e3 * e3) {
    swap(d2, d3);
    swap(e2, e3);
  }
  int n2 = d2 * d2 + e2 * e2;
  int n3 = d3 * d3 + e3 * e3;
  int x4 = x1, y4 = y1;
  bool ok = true;
  if (n2 < n3) {
    if (2 * n2 == n3 && d2 * d3 + e2 * e3 == n2) {
      x4 += d3 - d2;
      y4 += e3 - e2;
    } else {
      ok = false;
    }
  } else {
    if (n2 == n3 && d2 * d3 + e2 * e3 == 0) {
      x4 += d2 + d3;
      y4 += e2 + e3;
    } else {
      ok = false;
    }
  }
  if (ok) {
    cout << x4 << " " << y4 << endl;
  } else {
    cout << -1 << endl;
  }
}
