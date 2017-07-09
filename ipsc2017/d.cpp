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
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

VI funny;
set<int> fset;

bool boring(int v) {
  set<int> app;
  while (v > 0) {
    int w = v % 10;
    if (app.count(w)) {
      return true;
    }
    app.insert(w);
    v /= 10;
  }
  return false;
}

void init(void) {
  // Non-boring numbers up to 10000
  REP(i, 1, 10001) {
    if (not boring(i)) {
      funny.push_back(i);
      fset.insert(i);
    }
  }
}

VI solve(int n) {
  if (not boring(n)) {
    return VI(1, n);
  }
  REP(i, 0, funny.size()) {
    if (n <= funny[i]) {
      assert (0);
    }
    int diff = n - funny[i];
    if (not boring(diff)) {
      VI ret(1, diff);
      ret.push_back(funny[i]);
      return ret;
    }
  }
  assert (0);
}


int main(void){
  init();
  int t;
  cin >> t;
  while (t--) {
    int n;
    cin >> n;
    // optimal sol is <= 2. (easy, d1.in)
    VI sol = solve(n);
    cout << sol.size();
    REP(i, 0, sol.size()) {
      cout << " " << sol[i];
    }
    cout << endl;
  }
}
