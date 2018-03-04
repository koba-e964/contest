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
const ll mod = 1e9 + 7;

string s[10];

bool winning() {
  REP(i, 0, 10) {
    REP(j, 0, 10) {
      REP(dx, -1, 2) {
	REP(dy, -1, 2) {
	  if (dx == 0 && dy == 0) continue;
	  bool ok = true;
	  REP(k, 0, 5) {
	    int nx = i + k * dx;
	    int ny = j + k * dy;
	    if (nx < 0 || nx >= 10 || ny < 0 || ny >= 10) {
	      ok = false;
	      break;
	    }
	    if (s[nx][ny] != 'X') {
	      ok = false;
	      break;
	    }
	  }
	  if (ok) return true;
	}
      }
    }
  }
  return false;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  REP(i, 0, 10) cin >> s[i];
  REP(i, 0, 10) {
    REP(j, 0, 10) {
      if (s[i][j] != '.') continue;
      s[i][j] = 'X';
      if (winning()) {
	cout << "YES\n";
	return 0;
      }
      s[i][j] = '.';
    }
  }
  cout << "NO\n";
}
