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
typedef pair<int, int> PI;
const double EPS=1e-9;



int main(void){
  int kr,kb, ma = 0;
  cin >> kr >> kb;
  string s;
  cin >> s;
  int mask = 0;
  int white = 0;
  REP(i, 0, s.length()) {
    mask |= s[i] == 'W' ? 0 : (1 << i);
    white |= s[i] != 'W' ? 0 : (1 << i);    
  }
  for (int bits = mask; bits >= 0; --bits) {
    bits &= mask;
    string t;
    REP (i, 0, s.length()) {
      if ((bits | white) & (1 << i)) {
	t += s[i];
      }
    }
    bool ok = true;
    REP (i, 0, t.length() - kr) {
      if (t[i] == 'R' && t[i + kr] == 'R') {
	ok = false;
      }
    }
    REP (i, 0, t.length() - kb) {
      if (t[i] == 'B' && t[i + kb] == 'B') {
	ok = false;
      }
    }
    if (ok) {
      ma = max(ma, (int) t.length());
    }
  }
  cout << ma << endl;
}
