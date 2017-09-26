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

int calc(const string &cur) {
  set<int> c;
  REP(i, 0, cur.length()) {
    c.insert(cur[i]);
  }
  return c.size();
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  string s;
  cin >> s;
  // lowercase continuation
  int ma = 0;
  string cur;
  REP(i, 0, s.length() + 1) {
    if (i < (int) s.length() && s[i] >= 'a') {
      cur += s[i];
    } else {
      ma = max(ma, calc(cur));
      cur = "";
    }
  }
  cout << ma << "\n";
}
