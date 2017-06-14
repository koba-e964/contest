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


ll inf = 1e15;
int main(void){
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  REP(len, n, 4 * n) {
    // check
    VL t(len, -1);
    REP(i, 0, n) {
      t[i + len - n] = a[i];
    }
    set<ll> used;
    bool ok = true;
    for (int i = len - 1; i >= 0; --i) {
      ll tmp = inf;
      if (t[i] == -1) {
	if (2 * i + 1 < len) {
	  tmp = min(tmp, t[2 * i + 1]);
	}
	if (2 * i + 2 < len) {
	  tmp = min(tmp, t[2 * i + 2]);
	}
	while (used.count(tmp)) {
	  tmp--;
	  if (tmp < 0) {
	    ok = false;
	    break;
	  }
	}
      } else {
	tmp = t[i];
	if (2 * i + 1 < len) {
	  ok &= tmp < t[2 * i + 1];
	}
	if (2 * i + 2 < len) {
	  ok &= tmp < t[2 * i + 2];
	}
      }
      if (not ok) {
	break;
      }
      t[i] = tmp;
      used.insert(tmp);
    }
    if (ok) {
      cout << len << endl;
      return 0;
    }
  }
  cout << -1 << endl;
}
