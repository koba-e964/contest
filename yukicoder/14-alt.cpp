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
  int n;
  cin >> n;
  VI a(n);
  multiset<int> s;
  map<int, VI> factors;
  vector<multiset<int> > mul(10001);
  REP(i, 0, n) {
    cin >> a[i];
    VI fac;
    for (int j = 1; j * j <= a[i]; ++j) {
      if (a[i] % j) continue;
      fac.push_back(j);
      if (j * j != a[i]) fac.push_back(a[i] / j);
    }
    factors[a[i]] = fac;
    if (i > 0) {
      s.insert(a[i]);
      for (int f: fac) {
        mul[f].insert(a[i]);
      }
    }
  }
  REP(i, 0, n - 1) {
    PI mi(1e8, -1);
    for (int f: factors[a[i]]) {
      if (mul[f].empty()) continue;
      int ma = *mul[f].begin();
      mi = min(mi, PI(ma / f, ma));
    }
    int val = mi.second;
    a[i + 1] = val;
    // erase val from s, mul
    s.erase(s.find(val));
    for (int f: factors[val]) {
      mul[f].erase(mul[f].find(val));
    }
  }
  REP(i, 0, n) cout << a[i] << (i == n - 1 ? "\n" : " ");
}
