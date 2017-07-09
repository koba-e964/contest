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



int main(void){
  int t;
  cin >> t;
  while (t--) {
    cerr << "test " << t << endl;
    int c, ss;
    cin >> c >> ss;
    if (4 * ss >= c * c) {
      cout << 4 << endl;
      continue;
    }
    int s = sqrt(ss); // easy
    if (s * s != ss) {
      cout << "never" << endl;
      continue;
    }
    // simulate
    VI cell(c, 0); // 0: black, 1: white
    int cnt = 0;
    int pos = 0;
    while (true) {
      // flip
      REP(i, 0, s / 2) {
	int from = (pos + i) % c;
	int to = (pos + s - 1 - i) % c;
	int olda = cell[from];
	int oldb = cell[to];
	cell[from] = !oldb;
	cell[to] = !olda;
      }
      if (s % 2 == 1) {
	int v = (pos + s / 2) % c;
	cell[v] = !cell[v];
      }
      pos = (pos + s) % c;
      // is all black?
      cnt += 1;
      bool black = true;
      REP(i, 0, c) {
	black &= cell[i] == 0;
      }
      if (black) {
	break;
      }
    }
    if (1) {
      cout << "f(" << c << " " << s << ") = ";
    }
    cout << cnt << endl;
  }
}
