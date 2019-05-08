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


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k;
  while(cin >> n >> k && n && k) {
    VI c(n);
    REP(i, 0, n) cin >> c[i];
    if (k == 1) {
      cout << 0 << endl;
      continue;
    }
    int ma = 0;
    int mam = 0;
    REP(i, 0, n - k + 1) {
      int p = 1;
      REP(j, 0, k) p *= c[i + j];
      ma = max(ma, p);
    }
    REP(i, 0, n - k + 1) {
      int p = 0;
      REP(j, 0, n) {
        if (j < i || j >= i + k) p = max(p, c[j]);
      }
      int m = 1e8;
      REP(j, 0, k) {
        p *= c[i + j];
        m = min(m, c[i + j]);
      }
      mam = max(mam, p / m);
    }
    cout << max(0, mam - ma) << endl;
  }
}
