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

ll f(ll st, ll len) {
  if (st <= -len) {
    return (-st * 2 - len + 1) * len / 2;
  }
  if (st >= 0) {
    return (2 * st + len - 1) * len / 2;
  }
  return (st + len - 1) * (st + len) / 2 + (-st + 1) * (-st) / 2;
}

int main(void){
  int r, g, b;
  cin >> r >> g >> b;
  ll mi = 1e9;
  REP(gs, -900, 801) {
    ll mir = 1e9;
    REP(rs, -1300, gs - r + 1) {
      mir = min(mir, f(rs + 100, r));
    }
    ll mib = 1e9;
    REP(bs, gs + g, 1102) {
      mib = min(mib, f(bs - 100, b));
    }
    ll tot = mir + mib + f(gs, g);
    mi = min(mi, tot);
  }
  cout << mi << endl;
}
