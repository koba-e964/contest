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
  ios::sync_with_stdio(false);
  cin.tie(0);
  string sp, t;
  cin >> sp >> t;
  string mi = "}";
  REP(i, 0, sp.length() - t.length() + 1) {
    string s(sp);
    bool ok = true;
    REP(j, i, i + t.length()) {
      if (s[j] == '?') {
	s[j] = t[j - i];
      }
      if (s[j] != t[j - i]) {
	ok = false;
	break;
      }
    }
    if (not ok) {
      continue;
    }
    REP(i, 0, s.length()) {
      if (s[i] == '?') {
	s[i] = 'a';
      }
    }
    mi = min(mi, s);
  }
  cout << (mi == "}" ? "UNRESTORABLE" : mi) << "\n";
}
