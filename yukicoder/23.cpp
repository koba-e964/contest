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
  int h,a,d;
  cin >> h >> a >> d;
  int m = 123456789;
  REP(i, 0, (h + 2 * a) / a) {
    int r = h - a * i;
    int td = max((r + d - 1) / d, 0);
    m = min(m, 2 * i + td * 3);
  }
  cout << m / 2.0 << endl;
  return 0;
}
