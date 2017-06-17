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

const int N = 123;
int a[N][N];

int main(void){
  cin.tie(0);
  ios::sync_with_stdio(false);
  int n, m;
  cin >> n >> m;
  int mi = 1e6;
  REP(i, 0, n) {
    REP(j, 0, m) {
      cin >> a[i][j];
      mi = min(mi, a[i][j]);
    }
  }
  vector<PI> mvs;
  if (n < m) {
    REP(i, 0, mi) {
      REP(j, 0, n) {
	mvs.push_back(PI(0, j));
      }
    }
  } else {
    REP(i, 0, mi) {
      REP(j, 0, m) {
	mvs.push_back(PI(1, j));
      }
    }
  }
  REP(i, 0, n) {
    REP(j, 0, m) {
      a[i][j] -= mi;
    }
  }
  mi = 0;
  while (true) {
    // all zero?
    bool all_zero = true;
    REP(i, 0, n) {
      if (not all_zero) { break; }
      REP(j, 0, m) {
	if (a[i][j] != 0) {
	  all_zero = false;
	  break;
	}
      }
    }
    if (all_zero) {
      break;
    }
    // find operation, do it.
    bool found = false;
    REP(i, 0, n) {
      int mi = 1e6;
      REP(j, 0, m) {
	mi = min(mi, a[i][j]);
      }
      if (mi > 0) {
	found = true;
	REP(j, 0, mi) {
	  mvs.push_back(PI(0, i));
	}
	REP(j, 0, m) {
	  a[i][j] -= mi;
	}
	break;
      }
    }
    if (found) { continue; }
    REP(i, 0, m) {
      int mi = 1e6;
      REP(j, 0, n) {
	mi = min(mi, a[j][i]);
      }
      if (mi > 0) {
	found = true;
	REP(j, 0, mi) {
	  mvs.push_back(PI(1, i));
	}
	REP(j, 0, n) {
	  a[j][i] -= mi;
	}
	break;
      }
    }
    if (not found) {
      cout << -1 << endl;
      return 0;
    }
  }
  cout << mvs.size() << endl;
  REP(i, 0, mvs.size()) {
    string ops[2] = {"row", "col"};
    cout << ops[mvs[i].first] << " " << mvs[i].second + 1 << endl;
  }
}
