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


int dp[11][101][10010];
int a[3], db, dc, b[3], c[3];

int rec(int x, int y, int z) {
  if (x < 0 || x > 10 || y < 0 || y > 100 || z < 0 || z > 10000) {
    return 0;
  }
  int &ret = dp[x][y][z];
  if (ret >= 0) {
    return ret;
  }
  int ma = 0;
  REP(i, 0, db / 1000 + 1) {
    REP(j, 0, db / 100 - 10 * i + 1) {
      int r = db - 1000 * i - 100 * j;
      if (x >= i && y >= j && z >= r) {
	ma = max(ma, rec(x - i + b[0], y - j + b[1], z - r + b[2]) + 1);
      }
    }
  }
  REP(i, 0, dc / 1000 + 1) {
    REP(j, 0, dc / 100 - 10 * i + 1) {
      int r = dc - 1000 * i - 100 * j;
      if (x >= i && y >= j && z >= r) {
	ma = max(ma, rec(x - i + c[0], y - j + c[1], z - r + c[2]) + 1);
      }
    }
  }
  return ret = ma;
}

int main(void){
  REP(i, 0, 3) {
    cin >> a[i];
  }
  cin >> db;
  REP(i, 0, 3) {
    cin >> b[i];
  }
  cin >> dc;
  REP(i, 0, 3) {
    cin >> c[i];
  }
  memset(dp, -1, sizeof(dp));
  cout << rec(a[0], a[1], a[2]) << endl;
}
