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


// This solution is written after the author read the editorial.
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int a, b;
  cin >> a >> b;
  vector<VI> d(a, VI(b));
  REP(i, 0, a) {
    REP(j, 0, b) {
      cin >> d[i][j];
    }
  }
  vector<VI> f(101, VI(101, 0));
  REP(i, 0, a) {
    REP(j, 0, b) {
      REP(v, 0, 101) {
	REP(w, 0, 101) {
	  f[v][w] = max(f[v][w], d[i][j] - (i + 1) * v - (j + 1) * w);
	}
      }
    }
  }
  vector<VI> nd(a, VI(b, 1e8));
  REP(i, 0, a) {
    REP(j, 0, b) {
      REP(v, 0, 101) {
	REP(w, 0, 101) {
	  nd[i][j] = min(nd[i][j], f[v][w] + (i + 1) * v + (j + 1) * w);
	}
      }
    }
  }
  bool ok = true;
  if (nd != d) {
    ok = false;
  }
  if (ok) {
    REP(i, 0, 101) {
      REP(j, 0, 101) {
	if (f[i][j] < 0 || f[i][j] > 100) {
	  ok = false;
	}
      }
    }
  }
  if (ok) {
    cout << "Possible\n";
    int n = 202;
    int m = 101 * 101 + 2 * 100;
    cout << n << " " << m << "\n";
    REP(i, 0, 100) {
      cout << i + 1 << " " << i + 2 << " X\n";
    }
    REP(i, 0, 100) {
      cout << i + 103 << " " << i + 102 << " Y\n";
    }
    REP(i, 0, 101) {
      REP(j, 0, 101) {
	cout << i + 1 << " " << j + 102 << " " << f[i][j] << "\n";
      }
    }
    cout << 1 << " " << 102 << "\n";
  } else {
    cout << "Impossible\n";
  }
}
