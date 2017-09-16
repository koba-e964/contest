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
  int n;
  cin >> n;
  vector<VI> a(n, VI(n));
  vector<VI> dist(n, VI(n));
  REP(i, 0, n) {
    REP(j, 0, n) {
      cin >> a[i][j];
      dist[i][j] = a[i][j];
    }
  }
  REP(k, 0, n) {
    REP(i, 0, n) {
      REP(j, 0, n) {
	dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
      }
    }
  }
  if (a != dist) {
    cout << -1 << endl;
    return 0;
  }
  vector<vector<bool> > nec(n, vector<bool>(n, false));
  REP(i, 0, n) {
    REP(j, 0, n) {
      if (i == j) {
	continue;
      }
      // is i-j inexpendable?
      bool ex = false;
      REP(k, 0, n) {
	if (k == i || k == j) { continue; }
	if (a[i][j] == a[i][k] + a[k][j]) {
	  ex = true;
	  break;
	}
      }
      nec[i][j] = not ex;
    }
  }
  ll tot = 0;
  REP(i, 0, n) {
    REP(j, 0, i) {
      tot += nec[i][j] ? a[i][j] : 0;
    }
  }
  cout << tot << endl;
      
}
