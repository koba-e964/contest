#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int a, b, c, d;
  cin >> a >> b >> c >> d;
  int koba[4] = {a, b, c, d};
  int n = 50;
  int m = n / 2;
  string col = "ABCD";
  vector<string> ans(n, string(n, '+'));
  REP(i, 0, n) {
    REP(j, 0, n) {
      int idx = 2 * (i / m) + (j / m);
      ans[i][j] = col[idx];
    }
  }
  REP(d, 0, 4) {
    int bx = (d / 2) * m, by = (d % 2) * m;
    REP(i, 0, koba[(d + 1) % 4] - 1) {
      int x = 3 * (i / 23) + 1, y = i % 23;
      if (y % 2 == 1) x++;
      y++;
      ans[bx + x][by + y] = col[(d + 1) % 4];
    }
  }
  cout << n << " " << n << "\n";
  REP(i, 0, n) {
    cout << ans[i] << "\n";
  }
}
