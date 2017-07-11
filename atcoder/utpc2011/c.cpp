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

bool unsym(char a, char b) {
  map<char, char> t;
  t['i'] = 'i';
  t['w'] = 'w';
  t['('] = ')';
  t[')'] = '(';
  t['{'] = '}';
  t['}'] = '{';
  t['['] = ']';
  t[']'] = '[';
  return t[a] != b;
}

bool is_palin(const string &s) {
  REP(i, 0, (s.length() + 1) / 2) {
    if (unsym(s[i], s[s.length() - 1 - i])) {
      return false;
    }
  }
  int ic = 0;
  int wc = 0;
  REP(i, 0, s.length()) {
    if (s[i] == 'i') {
      ic += 1;
    }
    if (s[i] == 'w') {
      wc += 1;
    }
  }
  return ic == 2 && wc == 1;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string s;
  cin >> s;
  int n = s.length();
  int ma = 0;
  REP(bits, 0, 1 << n) {
    string t = "";
    REP(i, 0, n) {
      if (bits & 1 << i) {
	t += s[i];
      }
    }
    if (is_palin(t)) {
      ma = max(ma, __builtin_popcount(bits));
    }
  }
  cout << ma << "\n";
}
