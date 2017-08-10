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

int calc(const string &s, const string &pat) {
  int c = 0;
  REP(i, 0, s.length() + 1 - pat.length()) {
    if (s.substr(i, pat.length()) == pat) {
      c += 1;
    }
  }
  return c;
}

int main(void) {
  string s;
  cin >> s;
  cout << calc(s, "JOI") << endl;
  cout << calc(s, "IOI") << endl;
}
