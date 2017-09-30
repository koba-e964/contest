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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;

ll calc(PL pt) {
  ll x = pt.first;
  ll y = pt.second;
  x = abs(x);
  y = abs(y);
  ll z = x + y;
  ll cur = 0;
  ll idx = 0;
  while (true) {
    idx += 1;
    cur += idx;
    if (cur >= z && (cur - z) % 2 == 0) {
      return idx;
    }
  }
}

int main(void) {
  int n;
  cin >> n;
  vector<PL> pt;
  REP(i, 0, n) {
    int x, y;
    cin >> x >> y;
    pt.push_back(PL(x, y));
  }
  // parity
  {
    int bit[2] = {};
    REP(i, 0, n) {
      int b = (pt[i].first + pt[i].second) % 2 + 2;
      b %= 2;
      bit[b] += 1;
    }
    if (bit[0] != 0 && bit[1] != 0) {
      cout << -1 << endl;
      return 0;
    }
  }
  ll ma = 0;
  REP(i, 0, n) {
    ma = max(ma, calc(pt[i]));
  }
  cout << ma << endl;
}
