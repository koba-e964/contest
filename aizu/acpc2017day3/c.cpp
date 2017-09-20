#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <map>
#include <queue>
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

void fail(void) {
  cout << "no\n";
  exit(0);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string t, p;
  cin >> t >> p;
  t += '$';
  p += '$';
  int pos = 0;
  REP(i, 0, p.size()) {
    char prev = i >= 1 ? p[i - 1] : '\0';
    while (pos < (int) t.size()) {
      if (t[pos] == p[i]) {
	break;
      }
      if (t[pos] == prev) {
	fail();
      }
      pos++;
    }
    if (pos == (int) t.size()) {
      fail();
    }
    pos++;
  }
  cout << "yes\n";
}
