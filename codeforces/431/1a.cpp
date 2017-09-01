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

PI dexteram(int x) {
  REP(i, 1, 500) {
    int y = i * (i + 1) / 2;
    if (y > x) {
      return PI(i, x - (y - i));
    }
  }
  assert (0);
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int k;
  cin >> k;
  string s = "";
  char c = 'a';
  while (k > 0) {
    PI res = dexteram(k);
    REP(i, 0, res.first) {
      s += c;
    }
    k = res.second;
    c += 1;
  }
  if (s == "") {
    s = "a";
  }
  cout << s << endl;
}
