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

string s;

int ival(int x) {
  int r = 0;
  while (1) {
    if ('0' <= s[x + r] && s[x + r] <= '9') {
      r++;
    } else {
      break;
    }
  }
  return r ? r: -1;
}

int expr(int x) {
  if (s.substr(x, 2) == "{}") {
    return 2;
  }
  int pc = 1;
  int r = 1;
  if (s[x] != '{') {
    return ival(x);
  }
  while (pc > 0) {
    switch (s[x + r]) {
    case '{':
      pc++;
      break;
    case '}':
      pc--;
      break;
    }
    r++;
  }
  return r;
}

int dict(int x) {
  if (s.substr(x,2) == "{}") {
    return 2;
  }
  if (s[x] != '{') {
    return -1;
  }
  int res = expr(x + 1);
  if (res == -1) {
    return -1;
  }
  if (s[x + 1 + res] != ':') {
    return -1;
  }
  return 1;
}


int main(void){
  cin >> s;
  cout << (dict(0) == -1 ? "set" : "dict") << endl;
}
