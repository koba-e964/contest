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

const int W = 19;
int a[W][W];

int n,m,p,q,r;
int main(void){
  cin >> n >> m >> p >> q >> r;
  REP(i, 0, r) {
    int x,y,z;
    cin >> x >> y >> z;
    x--, y--;
    a[x][y] = z;
  }
  int ma = 0;
  REP(bits, 0, 1 << n) {
    if (__builtin_popcount(bits) != p) {
      continue;
    }
    int u[W] = {0};
    REP (i, 0, n) {
      if ((bits & (1 << i)) == 0) {
	continue;
      }
      REP(j, 0, m) {
	u[j] += a[i][j];
      }
    }
    sort(u, u + m);
    reverse(u, u + m);
    int sum = 0;
    REP(i, 0, q) {
      sum += u[i];
    }
    ma = max(ma, sum);
  }
  cout << ma << endl;
}
