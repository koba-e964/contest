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
typedef pair<int, int> PI;
const double EPS=1e-9;

string repl(string s) {
  string t;
  int p = 0;
  while (p < s.length()) {
    if (s.substr(p, 3) != "ABC") {
      t += s[p];
      p++;
      continue;
    }
    t += "x";
    p += 3;
  }
  return t;
}

int main(void){
  string s;
  cin >> s;
  while (s.length() > 3) {
    string ns = repl(s);
    if (ns.length() == s.length() || ns.length() < 3) {
      cout << "No" << endl;
      return 0;
    }
    int fl = 0;
    REP(i, 0, ns.length()) {
      if (ns[i] != 'x') {
	fl |= 1 << (ns[i] - 'A');
      }
    }
    if (fl == 3 || fl == 5 || fl == 6) {
      char tbl[8] = "xxxCxBA";
      replace(ns.begin(), ns.end(), 'x', tbl[fl]);
    } else {
      cout << "No" << endl;
      return 0;
    }
    s = ns;
  }
  cout << (s == "ABC" ? "Yes" : "No") << endl;
}
