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
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k;
  string s;
  cin >> n >> k >> s;
  char mi = '{';
  for (auto c: s) {
    mi = min(mi, c);
  }
  VI app(26);
  for (auto c: s) {
    app[c - 'a'] = 1;
  }
  if (k > n) {
    REP(i, n, k) {
      s += mi;
    }
    cout << s << "\n";
    return 0;
  }
  for (int i = k - 1; i >= 0; --i) {
    bool ok = 0;
    char c = -1;
    int ind = s[i] - 'a';
    REP(j, ind + 1, 26) {
      if (app[j]) {
	ok = 1;
	c = 'a' + j;
	break;
      }
    }
    if (ok) {
      string t = s.substr(0, i) + c + string(k - i - 1, mi);
      cout << t << "\n";
      return 0;
    }
  }
  assert (0);
}
