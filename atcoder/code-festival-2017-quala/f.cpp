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


ll calc(const VL &a) {
  int n = a.size();
  // all 1?
  bool all_one = true;
  REP(i, 0, n) {
    all_one &= a[i] == 1;
  }
  if (all_one) {
    return 0;
  }
  VL sub(n);
  ll tot = 0;
  int cur = 0;
  REP(i, 0, n) {
    sub[i] = (a[i] + 1) / 2;
    // 0 ... 0 1 -> ok
    // 1 0 ... 0 1 -> Ok
    // 1 0 ... 0 -> ok
    // if it encounters 1, this sector can be completely ignored
    if (a[i] == 1) {
      if (cur > 0) {
	tot += 1;
	cur = 0;
      }
      // else, nothing happens.
      continue;
    }
    if (a[i] % 2 == 1) {
      if (cur == 0) {
	cur = 1;
      } else {
	tot += 1;
	cur = 0;
      }
    } else {
      cur += 1;
    }
  }
  if (cur > 0) {
    tot += 1;
  }
  return tot + calc(sub);
}

map<VL, ll> memo;

ll exact(const VL &a, bool path_rec = false) {
  int n = a.size();
  pair<ll, PI> mi(5e15, PI(-1, -1));
  bool all_one = true;
  REP(i, 0, n) {
    all_one &= a[i] == 1;
  }
  if (all_one) {
    return 0;
  }
  if (not path_rec && memo.count(a)) {
    return memo[a];
  }
  REP(i, 0, n) {
    REP(j, i, n) {
      VL sub(a);
      bool ok = true;
      bool nonone = false;
      REP(k, i, j + 1) {
	sub[k] = (sub[k] + 1) / 2;
	if (k != i && k != j) {
	  if (a[k] % 2 != 0) {
	    ok = false;
	    break;
	  }
	}
	if (a[k] != 1) {
	  nonone = true;
	}
      }
      if (not ok || not nonone) {
	continue;
      }
      ll res = exact(sub);
      mi = min(mi, make_pair(res, PI(i, j)));
    }
  }
  if (path_rec) {
    int u = mi.second.first;
    int v = mi.second.second;
    cerr << "perform " << u << " " << v << endl;
    cerr << "a:";
    REP(i, 0, n) {
      cerr << " " << a[i];
    }
    cerr << endl;
    VL sub(a);
    REP(k, u, v + 1) {
      sub[k] = (sub[k] + 1) / 2;
    }
    exact(sub, true);
  }
  return memo[a] = mi.first + 1;
}


int main(void) {
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  cout << exact(a) << endl;
  exact(a, true);
}
