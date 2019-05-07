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
  int n;
  while (cin >> n && n) {
    VI x(3), y(3);
    REP(i, 0, n) {
      int h, m, t;
      cin >> h;
      cin.ignore();
      cin >> m >> t;
      int a = (t + 60 - m) % 60;
      int kind = -1;
      if (11 <= h && h <= 14) kind = 0;
      if (18 <= h && h <= 20) kind = 1;
      if (21 <= h || h <= 1) kind = 2;
      if (kind >= 0) {
        x[kind] += a <= 8;
        y[kind] += 1;
      }
    }
    string kind[3] = {"lunch", "dinner", "midnight"};
    REP(i, 0, 3) {
      cout << kind[i] << " ";
      if (y[i]) {
        cout << 100 * x[i] / y[i] << endl;
      } else {
        cout << "no guest\n";
      }
    }
  }
}
