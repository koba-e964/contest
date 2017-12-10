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
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL a(n);
  ll ma = -1;
  int maxi = -1;
  REP(i, 0, n) {
    cin >> a[i];
    if (ma < abs(a[i])) {
      ma = abs(a[i]);
      maxi = i;
    }
  }
  if (ma == 0) {
    cout << 0 << endl;
    return 0;
  }
  vector<PI> ops;
  if (a[maxi] > 0) {
    ops.push_back(PI(maxi, 0));
    ops.push_back(PI(maxi, 0));
    REP(i, 0, n - 1) {
      ops.push_back(PI(i, i + 1));
      ops.push_back(PI(i, i + 1));
    }
  } else {
    ops.push_back(PI(maxi, n - 1));
    ops.push_back(PI(maxi, n - 1));
    for (int i = n - 1; i >= 1; --i) {
      ops.push_back(PI(i, i - 1));
      ops.push_back(PI(i, i - 1));
    }
  }
  cout << ops.size() << endl;
  REP(i, 0, ops.size()) {
    cout << ops[i].first + 1 << " " << ops[i].second + 1 << "\n";
  }
}
