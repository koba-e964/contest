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
    vector<VI> a(n, VI(n));
    REP(i, 0, n) REP(j, 0, n) cin >> a[i][j];
    vector<VI> fl(n, VI(n));
    REP(i, 0, n) {
      int mi = 1e8;
      REP(j, 0, n) mi = min(mi, a[i][j]);
      REP(j, 0, n) fl[i][j] |= mi == a[i][j] ? 1 : 0;
    }
    REP(j, 0, n) {
      int ma = 0;
      REP(i, 0, n) ma = max(ma, a[i][j]);
      REP(i, 0, n) fl[i][j] |= ma == a[i][j] ? 2 : 0;
    }
    int c = 0;
    REP(i, 0, n) REP(j, 0, n) {
      if (fl[i][j] == 3) c = a[i][j];
    }
    cout << c << endl;
  }
}
