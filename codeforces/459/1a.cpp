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

const int DEBUG = 0;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string s;
  cin >> s;
  int n = s.length();
  ll tot = 0;
  REP(i, 0, n) {
    int delta = 0;
    int que = 0;
    bool ok = true;
    REP(j, i, n) {
      if (s[j] == '?') {
	que += 1;
      } else if (s[j] == '(') {
	delta += 1;
      } else {
	delta -= 1;
      }
      while (que > 0 && delta < que) {
	que -= 1;
	delta += 1;
      }
      if (delta < -que) {
	ok = false;
	break;
      }
      if (DEBUG) {
	DEBUGP(delta);
	DEBUGP(que);
      }
      if ((delta + que) % 2 == 0 && ok && delta <= que) {
	if (DEBUG) {
	  cerr << "valid: " << i << " " << j << endl;
	}
	tot += 1;
      }
    }
  }
  cout << tot << "\n";
}
