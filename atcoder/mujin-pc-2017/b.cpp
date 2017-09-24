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


// The author implemented this solution after he read the editorial.
int main(void) {
  int n;
  cin >> n;
  vector<string> s(n);
  REP(i, 0, n) {
    cin >> s[i];
  }
  int c = 0;
  int bc = 0;
  REP(i, 0, n) {
    bool all_black = true;
    bool all_white = true;
    REP(j, 0, n) {
      if (s[j][i] == '.') {
	all_black = false;
      }
      if (s[j][i] == '#') {
	all_white = false;
      }
    }
    c += all_black ? 0 : 1;
    bc += all_white ? 0 : 1;
  }
  if (bc == 0) {
    cout << -1 << endl;
    return 0;
  }
  int mi = 1e8;
  REP(i, 0, n) {
    bool all_white = true;
    REP(j, 0, n) {
      if (s[j][i] == '#') {
	all_white = false;
	break;
      }
    }
    int x = 0;
    REP(j, 0, n) {
      x += s[i][j] == '.' ? 1 : 0;
    }
    if (all_white) {
      x += 1;
    }
    mi = min(mi, x);
  }
  cout << mi + c << endl;
}
