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


int solve_naive(const vector<VI> &a) {
  int n = a.size();
  assert (n <= 4);
  int mi = 1e8;
  REP(bits, 0, 1 << (n * n)) {
    vector<VI> b(a);
    REP(i, 0, n * n) {
      if (bits & 1 << i) {
	int x = i / n;
	int y = i % n;
	b[x][y] = 0;
      }
    }
    bool ok = true;
    REP(i, 0, n) {
      if (not ok) break;
      REP(j, 0, n) {
	if (not ok) break;
	if (b[i][j] == 0) continue;
	REP(k, 0, j) {
	  if (b[i][j] == b[i][k]) {
	    ok = false;
	    break;
	  }
	}
      }
    }
    REP(i, 0, n) {
      if (not ok) break;
      REP(j, 0, n) {
	if (not ok) break;
	if (b[j][i] == 0) continue;
	REP(k, 0, j) {
	  if (b[j][i] == b[k][i]) {
	    ok = false;
	    break;
	  }
	}
      }
    }
    if (ok) {
      mi = min(mi, __builtin_popcount(bits));
    }
  }
  return mi;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  REP(case_nr, 1, t + 1) {
    int n;
    cin >> n;
    vector<VI> a(n, VI(n));
    REP(i, 0, n) REP(j, 0, n) cin >> a[i][j];
    int ans = solve_naive(a);
    cout << "Case #" << case_nr << ": " << ans << endl;
  }
}
