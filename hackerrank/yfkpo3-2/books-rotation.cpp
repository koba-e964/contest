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

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int h, w;
  cin >> h >> w;
  vector<VI> a(h, VI(w));
  REP(i, 0, h) REP(j, 0, w) cin >> a[i][j];
  int ans = 0;
  REP(j, 1, w) {
    bool some = 0;
    REP(i, 0, h) {
      bool ok = 1;
      REP(k, 0, h) {
        if (a[(i + k) % h][j] != a[k][0]) ok = 0;
      }
      if (ok) some = 1;
    }
    ans += some;
  }
  cout << (ans == w - 1 ? "Yes" : "No") << endl;
}
