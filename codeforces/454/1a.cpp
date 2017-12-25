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
  int n;
  cin >> n;
  set<char> unsafe;
  int ex = 0;
  REP(i, 'a', 'z' + 1) {
    unsafe.insert((char) i);
  }
  REP(i, 0, n) {
    string ty, s;
    cin >> ty >> s;
    if (unsafe.size() == 1 && (ty == "!" || (ty == "?" && i < n - 1))) {
      ex += 1;
    }
    if (ty == "." || (i < n - 1 && ty == "?")) {
      REP(j, 0, s.length()) {
	unsafe.erase(s[j]);
      }
    } else {
      if (0) {
	cerr << "cand:";
	for (char ee: unsafe) cerr << " " << ee;
	cerr << endl;
      }
      set<char> nc;
      REP(j, 0, s.length()) {
	if (unsafe.count(s[j])) {
	  nc.insert(s[j]);
	}
      }
      unsafe = nc;
    }
  }
  cout << ex << "\n";
}
