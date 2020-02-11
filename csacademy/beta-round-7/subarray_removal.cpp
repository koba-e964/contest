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

// B
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL a(n);
  VL acc(n + 1);
  REP(i, 0, n) {
    cin >> a[i];
    acc[i + 1] = acc[i] + a[i];
  }
  VL lft(n + 1);
  VL rgt(n + 1);
  ll mi = 1e17;
  REP(i, 0, n + 1) {
    mi = min(mi, acc[i]);
    lft[i] = acc[i] - mi;
  }
  ll ma = -1e17;
  for (int i = n; i >= 0; i--) {
    ma = max(ma, acc[i]);
    rgt[i] = ma - acc[i];
  }
  ma = -1e17;
  ll gma = -1e17;
  REP(i, 0, n + 1) {
    gma = max(gma, ma + rgt[i]);
    ma = max(ma, lft[i]);
  }
  if (gma == 0) {
    gma = -1e17;
    REP(i, 0, n) {
      gma = max(gma, a[i]);
    }
  }
  cout << gma << endl;
}
