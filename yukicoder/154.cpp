#include <algorithm>
#include <bitset>
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
typedef pair<int, int> PI;
const double EPS=1e-9;

bool solve(const string &s) {
  int n = s.length();
  int w = 0, g = 0, r = 0;
  REP(i, 0, n) {
    switch(s[i]) {
    case 'W':
      w++; break;
    case 'G':
      g++; break;
    case 'R':
      r++; break;
    default:
      return 0;
    }
    if (w < g || w < r || g < r) {
      return 0;
    }
  }
  if (g != r || g == 0) {
    return 0;
  }
  w = g = r = 0;
  for (int i = n - 1; i >= 0; --i) {
    switch(s[i]) {
    case 'W':
      w++; break;
    case 'G':
      g++; break;
    case 'R':
      r++; break;
    default:
      return 0;
    }
    if (g > r || (g == 0 && w > 0)) {
      return 0;
    }
  }
  return 1;
}

int main(void){
  int t;
  cin >> t;
  REP (loop_var, 0, t) {
    string s;
    cin >> s;
    cout << (solve(s) ? "possible" : "impossible") << endl;
  }
}
