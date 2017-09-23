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
  int h, w, d;
  cin >> h >> w >> d;
  if (d % 2 == 1) {
    // ITIMATU
    REP(i, 0, h) {
      string s(w, 'R');
      REP(j, 0, w) {
	if ((i + j) % 2 == 0) {
	  s[j] = 'B';
	}
      }
      cout << s << "\n";
    }
    return 0;
  }
  REP(i, 0, h) {
    string s(w, '.');
    REP(j, 0, w) {
      int p = (i - j + 500 * d) % (2 * d);
      int q = (i + j) % (2 * d);
      p /= d;
      q /= d;
      assert (p <= 1);
      assert (q <= 1);
      s[j] = "RGYB"[2 * p + q];
    }
    cout << s << "\n";
  }
}
