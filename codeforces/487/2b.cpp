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
  int n, p;
  cin >> n >> p;
  string s;
  cin >> s;
  bool ok = false;
  REP(i, 0, n - p) {
    if (s[i] == '.' && s[i + p] == '.') {
      s[i] = '0';
      s[i + p] = '1';
      ok = true;
      break;
    }
    if (s[i] == '.') {
      s[i] = '0' + '1' - s[i + p];
      ok = true;
      break;
    }
    if (s[i + p] == '.') {
      s[i + p] = '0' + '1' - s[i];
      ok = true;
      break;
    }
    if (s[i] != s[i + p]) {
      ok = true;
      break;
    }
  }
  if (not ok) {
    cout << "No\n";
    return 0;
  }
  // finalise
  REP(i, 0, n) {
    if (s[i] == '.') s[i] = '0';
  }
  cout << s << "\n";
}
