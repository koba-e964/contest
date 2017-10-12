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


class DominoTiling {
  int n, m;
  vector<string> grid;
  vector<VL> memo;
  vector<bool> dat;
  ll rec(int r, int bits) {
    if (r >= n) {
      return bits == 0 ? 1 : 0;
    }
    ll &ret = memo[r][bits];
    if (ret >= 0) {
      return ret;
    }
    // Check all constraints are met
    ll rest = (1 << m) - 1;
    REP(i, 0, m) {
      if ((bits & 1 << i) != 0 && grid[r][i] == 'X') {
	return ret = 0;
      }
      if ((bits & 1 << i) != 0 || grid[r][i] == 'X') {
	rest ^= 1 << i;
      }
    }
    ll tot = 0;
    int v = rest; // horizontally placed dominos
    while (true) {
      if (dat[v]) {
	tot += rec(r + 1, rest ^ v);
      }
      if (v == 0) {
	break;
      }
      v = (v - 1) & rest;
    }
    // cerr << "rec(" << r << " " << bits << ")=" << tot << endl;
    return ret = tot;
  }
  void enumerate(void) {
    this->dat = vector<bool>(1 << m);
    this->dat[0] = true;
    REP(bits, 1, 1 << m) {
      REP(i, 0, m - 1) {
	if ((bits & 3 << i) == (3 << i)) {
	  if (this->dat[bits ^ 3 << i]) {
	    this->dat[bits] = true;
	    break;
	  }
	}
      }
    }
  }
public:
  long long count(vector <string> grid) {
    n = grid.size();
    m = grid[0].size();
    this->grid = grid;
    this->memo = vector<VL>(n + 1, VL(1 << m, -1));
    this->enumerate();
    return rec(0, 0);
  }
};

int main(void) {
  int n;
  cin >> n;
  vector<string> grid(n);
  REP(i, 0, n) {
    cin >> grid[i];
  }
  cout << DominoTiling().count(grid) << endl;
}
