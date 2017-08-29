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
  int n;
  string s;
  cin >> n >> s;
  int bal = 0;
  int balmi = 0;
  REP(i, 0, n) {
    if (s[i] == '(') {
      bal += 1;
    } else {
      bal -= 1;
    }
    balmi = min(balmi, bal);
  }
  REP(i, 0, -balmi) {
    s = "(" + s;
  }
  bal = 0;
  REP(i, 0, s.length()) {
    if (s[i] == '(') {
      bal += 1;
    } else {
      bal -= 1;
    }
  }
  assert (bal >= 0);
  REP(i, 0, bal) {
    s = s + ")";
  }
  cout << s << endl;
      
}
