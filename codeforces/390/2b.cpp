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

bool solve(const string s[4]) {
  int dx[4] = {1, 1, 0, -1};
  int dy[4] = {0, 1, 1, 1};
  REP(i, 0, 4) {
    REP(j, 0, 4) {
      REP(d, 0, 4) {
	int ex = i + dx[d] * 2;
	int ey = j + dy[d] * 2;
	if (0 <= ex && ex < 4 && 0 <= ey && ey < 4) {
	  bool ok = true;
	  REP(k, 0, 3) {
	    ok &= s[i + dx[d] * k][j + dy[d] * k] == 'x';
	  }
	  if (ok) return true;
	}
      }
    }
  }
  return false;
}

int main(void){
  string s[4];
  REP(i, 0, 4) {
    cin >> s[i];
  }
  bool win = false;
  REP(i, 0, 4) {
    REP(j, 0, 4) {
      if (s[i][j] != '.') { continue; }
      s[i][j] = 'x';
      win |= solve(s);
      s[i][j] = '.';
    }
  }
  cout << (win ? "YES" : "NO") << endl;
}
