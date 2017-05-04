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

void fail(void) {
  cout << "NO" << endl;
  exit(0);
}

int main(void){
  int n;
  cin >> n;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  int k;
  cin >> k;
  VI b(k);
  REP(i, 0, k) {
    cin >> b[i];
  }
  VI cor(k + 1, 0);
  int pos = 0;
  int cur = b[0];
  REP(i, 0, n) {
    cur -= a[i];
    if (cur < 0) {
      fail();
    }
    if (cur == 0) {
      cor[pos + 1] = i + 1;
      pos += 1;
      if (pos < k) {
	cur = b[pos];
      }
    }
  }
  if (pos < k) {
    fail();
  }
  VI ops;
  REP(i, 0, k) {
    int lo = cor[i];
    int hi = cor[i + 1];
    // [lo, hi) all equal?
    if (hi - lo == 1) {
      continue;
    }
    int ma = -1, mi = 1e9;
    REP(j, lo, hi) {
      ma = max(ma, a[j]);
      mi = min(mi, a[j]);
    }
    if (ma == mi) {
      fail();
    }
    int piv = -1;
    int dir = -1;
    REP(j, lo, hi) {
      if (a[j] == ma &&
	  ((j < hi - 1 && a[j + 1] != ma)
	   || (j >= lo + 1 && a[j - 1] != ma))) {
	piv = j;
	if (j < hi - 1 && a[j + 1] != ma) {
	  dir = 1;
	} else {
	  dir = -1;
	}
      }
    }
    assert (piv >= 0);
    if (dir == -1) {
      REP(j, 0, piv - lo) {
	ops.push_back(-(piv - j + 1 - lo + i));
      }
      REP(j, 0, hi - piv - 1) {
	ops.push_back(i + 1);
      }
    } else {
      REP(j, 0, hi - piv - 1) {
	ops.push_back(i + 1 + piv - lo);
      }
      REP(j, 0, piv - lo) {
	ops.push_back(-(piv - j + 1 - lo + i));
      }
    }
  }
  cout << "YES" << endl;
  REP(i, 0, ops.size()) {
    int o = ops[i];
    if (o < 0) {
      cout << -o << " L" << endl;
    } else {
      cout << o << " R" << endl;
    }
  }
}
