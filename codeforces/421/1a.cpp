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



int main(void){
  int a, b;
  ll l, r;
  cin >> a >> b >> l >> r;
  l--, r--;
  VI ret(2 * a + 2 * b);
  if (a <= b) {
    REP(i, 0, a) {
      ret[i] = i;
    }
    REP(i, a, a + b) {
      ret[i] = a - 1;
    }
    REP(i, 0, a - 1) {
      ret[a + b + i] = i;
    }
    REP(i, 0, b + 1) {
      ret[2 * a + b - 1 + i] = a;
    }
  } else {
    REP(i, 0, a) {
      ret[i] = i;
    }
    REP(i, a, a + b) {
      ret[i] = a - 1;
    }
    REP(i, 0, b) {
      ret[a + b + i] = i;
    }
    REP(i, 0, a - b) {
      ret[a + 2 * b + i] = a + i;
    }
    REP(i, 0, b) {
      ret[2 * a + b + i] = 2 * a - b - 1;
    }
  }
  if (0) {
    REP(i, 0, ret.size()) {
      cerr << " " << ret[i];
    }
    cerr << endl;
  }
  if (r - l >= 4 * (a + b)) {
    set<int> tmp(ret.begin(), ret.end());
    cout << tmp.size() << endl;
    return 0;
  }
  set<int> accio;
  REP(i, l, r + 1) {
    accio.insert(ret[i % (2 * a + 2 * b)]);
  }
  cout << accio.size() << endl;
}
